#[cfg(feature = "ai")]
use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, Role}};
#[cfg(feature = "ai")]
use serde_json::json;
use crate::types::RustFileSnapshot;

pub struct AILinter {
    provider: AIProvider,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Debug, Clone)]
pub enum AIProvider {
    OpenAI { model: String },
    Google { model: String },
}

pub struct AIAnalysis {
    pub insights: Vec<String>,
    pub suggestions: Vec<String>,
    pub quality_score: Option<f32>,
}

impl AILinter {
    pub fn new(provider: AIProvider) -> Self {
        Self {
            provider,
            max_tokens: 4000,
            temperature: 0.3,
        }
    }

    /// Analyze entire project in one batched call to maximize context window usage
    #[cfg(feature = "ai")]
    pub async fn analyze_project(&self, snapshots: &[RustFileSnapshot]) -> Result<AIAnalysis, String> {
        // Build comprehensive project context
        let project_context = self.build_project_context(snapshots);
        
        match &self.provider {
            AIProvider::OpenAI { model } => self.analyze_with_openai(&project_context, model).await,
            AIProvider::Google { model } => self.analyze_with_google(&project_context, model).await,
        }
    }

    /// Explain code in layman's terms for non-technical users
    #[cfg(feature = "ai")]
    pub async fn explain_for_layman(&self, snapshots: &[RustFileSnapshot]) -> Result<String, String> {
        let context = self.build_layman_context(snapshots);
        
        match &self.provider {
            AIProvider::OpenAI { model } => self.explain_with_openai(&context, model).await,
            AIProvider::Google { model } => self.explain_with_google(&context, model).await,
        }
    }

    #[cfg(not(feature = "ai"))]
    pub async fn analyze_project(&self, _snapshots: &[RustFileSnapshot]) -> Result<AIAnalysis, String> {
        Err("AI features are not enabled. Compile with --features ai".to_string())
    }

    #[cfg(not(feature = "ai"))]
    pub async fn explain_for_layman(&self, _snapshots: &[RustFileSnapshot]) -> Result<String, String> {
        Err("AI features are not enabled. Compile with --features ai".to_string())
    }

    /// Build a comprehensive, context-window-maximizing prompt
    fn build_project_context(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut context = String::new();
        
        // Project statistics
        let total_functions: usize = snapshots.iter().map(|s| s.functions.len()).sum();
        let total_structs: usize = snapshots.iter().map(|s| s.structs.len()).sum();
        let total_enums: usize = snapshots.iter().map(|s| s.enums.len()).sum();
        
        context.push_str(&format!("# Rust Project Analysis Request\n\n"));
        context.push_str(&format!("## Project Overview\n"));
        context.push_str(&format!("- Files: {}\n", snapshots.len()));
        context.push_str(&format!("- Functions: {}\n", total_functions));
        context.push_str(&format!("- Structs: {}\n", total_structs));
        context.push_str(&format!("- Enums: {}\n", total_enums));
        context.push_str("\n## Code Structure\n\n");
        
        // Include all code details in one batch
        for snapshot in snapshots {
            context.push_str(&format!("### File: {}\n\n", snapshot.path));
            
            if !snapshot.functions.is_empty() {
                context.push_str("**Functions:**\n");
                for func in &snapshot.functions {
                    context.push_str(&format!(
                        "- `{}({})` - {} variables\n",
                        func.name,
                        func.args.join(", "),
                        func.variables.len()
                    ));
                }
                context.push('\n');
            }
            
            if !snapshot.structs.is_empty() {
                context.push_str("**Structs:**\n");
                for strct in &snapshot.structs {
                    context.push_str(&format!(
                        "- `{}` - {} fields, {} methods\n",
                        strct.name,
                        strct.fields.len(),
                        strct.methods.len()
                    ));
                }
                context.push('\n');
            }
            
            if !snapshot.enums.is_empty() {
                context.push_str("**Enums:**\n");
                for enm in &snapshot.enums {
                    context.push_str(&format!(
                        "- `{}` - {} variants\n",
                        enm.name,
                        enm.variants.len()
                    ));
                }
                context.push('\n');
            }
        }
        
        // Comprehensive analysis prompt
        context.push_str("\n## Analysis Request\n\n");
        context.push_str("Provide a comprehensive code quality analysis including:\n");
        context.push_str("1. **Architecture insights**: Overall design patterns and structure\n");
        context.push_str("2. **Code quality suggestions**: Naming, complexity, best practices\n");
        context.push_str("3. **Potential improvements**: Refactoring opportunities, missing abstractions\n");
        context.push_str("4. **Anti-patterns**: Any detected code smells or anti-patterns\n");
        context.push_str("5. **Quality score**: Rate the codebase from 0-100\n\n");
        context.push_str("Focus on actionable, specific suggestions. Be concise but thorough.\n");
        
        context
    }

    #[cfg(feature = "ai")]
    async fn analyze_with_openai(&self, context: &str, model: &str) -> Result<AIAnalysis, String> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set".to_string())?;
        
        let client = Client::new().with_api_key(api_key);
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(vec![
                ChatCompletionRequestMessage {
                    role: Role::System,
                    content: Some("You are an expert Rust code reviewer and architect. Provide specific, actionable insights.".to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                    function_call: None,
                },
                ChatCompletionRequestMessage {
                    role: Role::User,
                    content: Some(context.to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                    function_call: None,
                },
            ])
            .max_tokens(self.max_tokens as u32)
            .temperature(self.temperature)
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;
        
        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| format!("OpenAI API error: {}", e))?;
        
        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        
        Ok(self.parse_ai_response(&content))
    }

    #[cfg(feature = "ai")]
    async fn analyze_with_google(&self, context: &str, model: &str) -> Result<AIAnalysis, String> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| "GOOGLE_API_KEY environment variable not set".to_string())?;
        
        let client = reqwest::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        
        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": format!("You are an expert Rust code reviewer. {}", context)
                }]
            }],
            "generationConfig": {
                "temperature": self.temperature,
                "maxOutputTokens": self.max_tokens,
            }
        });
        
        let response = client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Google API request failed: {}", e))?;
        
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let content = response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        Ok(self.parse_ai_response(&content))
    }

    fn parse_ai_response(&self, content: &str) -> AIAnalysis {
        let mut insights = Vec::new();
        let mut suggestions = Vec::new();
        let mut quality_score = None;
        
        // Parse structured response
        for line in content.lines() {
            let line = line.trim();
            
            // Extract quality score
            if line.contains("score") && line.contains(char::is_numeric) {
                if let Some(score_str) = line.split_whitespace()
                    .find(|s| s.parse::<f32>().is_ok())
                {
                    quality_score = score_str.parse().ok();
                }
            }
            
            // Extract insights and suggestions
            if line.starts_with("- ") || line.starts_with("* ") {
                let item = line[2..].to_string();
                if content.contains("suggestion") || content.contains("improve") {
                    suggestions.push(item.clone());
                }
                insights.push(item);
            }
        }
        
        // If no structured parse, add entire content as one insight
        if insights.is_empty() {
            insights.push(content.to_string());
        }
        
        AIAnalysis {
            insights,
            suggestions,
            quality_score,
        }
    }

    /// Build layman-friendly context with focus on purpose and functionality
    fn build_layman_context(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut context = String::new();
        
        context.push_str("# Explain This Codebase in Simple Terms\n\n");
        context.push_str("You are explaining code to someone with NO programming experience.\n");
        context.push_str("Use analogies, simple language, and focus on WHAT it does and WHY.\n\n");
        
        context.push_str("## Project Structure\n\n");
        
        for snapshot in snapshots {
            context.push_str(&format!("### File: {}\n\n", snapshot.path));
            context.push_str("**What this file contains:**\n\n");
            
            // Explain functions
            if !snapshot.functions.is_empty() {
                context.push_str(&format!("This file has {} functions (tasks the program can do):\n\n", snapshot.functions.len()));
                for func in &snapshot.functions {
                    context.push_str(&format!(
                        "- `{}`: Takes {} input{}, processes data\n",
                        func.name,
                        func.args.len(),
                        if func.args.len() == 1 { "" } else { "s" }
                    ));
                }
                context.push('\n');
            }
            
            // Explain structs
            if !snapshot.structs.is_empty() {
                context.push_str(&format!("This file defines {} data structure{}:\n\n", 
                    snapshot.structs.len(),
                    if snapshot.structs.len() == 1 { "" } else { "s" }
                ));
                for strct in &snapshot.structs {
                    context.push_str(&format!(
                        "- `{}`: A container with {} piece{} of information\n",
                        strct.name,
                        strct.fields.len(),
                        if strct.fields.len() == 1 { "" } else { "s" }
                    ));
                }
                context.push('\n');
            }
            
            // Explain enums
            if !snapshot.enums.is_empty() {
                for enm in &snapshot.enums {
                    context.push_str(&format!(
                        "- `{}`: Represents {} different possible states or types\n",
                        enm.name,
                        enm.variants.len()
                    ));
                }
                context.push('\n');
            }
        }
        
        context.push_str("\n## Your Task\n\n");
        context.push_str("For EACH file, explain:\n\n");
        context.push_str("1. **Purpose**: What is this file's job in simple terms?\n");
        context.push_str("2. **Functionality**: What does it actually DO? (use real-world analogies)\n");
        context.push_str("3. **Key Components**: What are the main building blocks?\n");
        context.push_str("4. **How It Works**: Describe the logic flow in simple steps\n\n");
        context.push_str("Rules:\n");
        context.push_str("- NO jargon (avoid terms like 'instantiate', 'iterate', 'polymorphism')\n");
        context.push_str("- USE analogies (e.g., 'like a recipe', 'like a filing cabinet')\n");
        context.push_str("- Focus on PURPOSE, not syntax\n");
        context.push_str("- Explain as if talking to a curious 12-year-old\n");
        context.push_str("- Use emojis to make it engaging\n\n");
        
        context
    }

    #[cfg(feature = "ai")]
    async fn explain_with_openai(&self, context: &str, model: &str) -> Result<String, String> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set".to_string())?;
        
        let client = Client::new().with_api_key(api_key);
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(vec![
                ChatCompletionRequestMessage {
                    role: Role::System,
                    content: Some("You are a friendly teacher explaining programming to absolute beginners. Use simple language, analogies, and emojis.".to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                    function_call: None,
                },
                ChatCompletionRequestMessage {
                    role: Role::User,
                    content: Some(context.to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                    function_call: None,
                },
            ])
            .max_tokens(self.max_tokens as u32)
            .temperature(0.7) // Higher temperature for more creative explanations
            .build()
            .map_err(|e| format!("Failed to build request: {}", e))?;
        
        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| format!("OpenAI API error: {}", e))?;
        
        Ok(response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default())
    }

    #[cfg(feature = "ai")]
    async fn explain_with_google(&self, context: &str, model: &str) -> Result<String, String> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| "GOOGLE_API_KEY environment variable not set".to_string())?;
        
        let client = reqwest::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        
        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": format!("You are a friendly teacher. {}", context)
                }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": self.max_tokens,
            }
        });
        
        let response = client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Google API request failed: {}", e))?;
        
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
