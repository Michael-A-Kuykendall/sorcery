//! Prompt Templates - Chat Formatting
//! 
//! Rehydrated from: templates.spell
//! 
//! Different prompt formatting standards for various model families.

// ═══════════════════════════════════════════════════════════════════
// TemplateFamily - Prompt Format Standards
// From templates.spell: @TemplateFamily
// ═══════════════════════════════════════════════════════════════════

/// Represents different prompt formatting standards.
#[derive(Debug, Clone, Copy)]
pub enum TemplateFamily {
    ChatML,     // Qwen, ChatGLM, Phi
    Llama3,     // Meta Llama 3 family
    OpenChat,   // OpenChat, generic fallback
    Mistral,    // Mistral/Mixtral
}

impl TemplateFamily {
    /// Render conversation into model-specific prompt
    pub fn render(
        &self,
        system: Option<&str>,
        history: &[(String, String)],
        user_input: Option<&str>,
    ) -> String {
        match self {
            TemplateFamily::ChatML => render_chatml(system, history, user_input),
            TemplateFamily::Llama3 => render_llama3(system, history, user_input),
            TemplateFamily::OpenChat => render_openchat(system, history, user_input),
            TemplateFamily::Mistral => render_mistral(system, history, user_input),
        }
    }

    /// Returns default stop sequences for this template
    pub fn stop_tokens(&self) -> Vec<String> {
        match self {
            TemplateFamily::ChatML => vec![
                "<|im_end|>".to_string(),
                "<|im_start|>".to_string(),
            ],
            TemplateFamily::Llama3 => vec![
                "<|eot_id|>".to_string(),
                "<|end_of_text|>".to_string(),
            ],
            TemplateFamily::OpenChat => vec![
                "<|end_of_turn|>".to_string(),
            ],
            TemplateFamily::Mistral => vec![
                "</s>".to_string(),
                "[INST]".to_string(),
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ChatML Format
// From templates.spell: Used by Qwen, ChatGLM, Phi-3
// ═══════════════════════════════════════════════════════════════════

fn render_chatml(
    system: Option<&str>,
    history: &[(String, String)],
    user_input: Option<&str>,
) -> String {
    let mut prompt = String::new();
    
    // System message
    if let Some(sys) = system {
        prompt.push_str("<|im_start|>system\n");
        prompt.push_str(sys);
        prompt.push_str("\n<|im_end|>\n");
    }
    
    // History
    for (user, assistant) in history {
        prompt.push_str("<|im_start|>user\n");
        prompt.push_str(user);
        prompt.push_str("\n<|im_end|>\n");
        prompt.push_str("<|im_start|>assistant\n");
        prompt.push_str(assistant);
        prompt.push_str("\n<|im_end|>\n");
    }
    
    // Current input
    if let Some(input) = user_input {
        prompt.push_str("<|im_start|>user\n");
        prompt.push_str(input);
        prompt.push_str("\n<|im_end|>\n");
    }
    
    // Prompt for assistant response
    prompt.push_str("<|im_start|>assistant\n");
    
    prompt
}

// ═══════════════════════════════════════════════════════════════════
// Llama 3 Format
// From templates.spell: Meta's Llama 3 instruction format
// ═══════════════════════════════════════════════════════════════════

fn render_llama3(
    system: Option<&str>,
    history: &[(String, String)],
    user_input: Option<&str>,
) -> String {
    let mut prompt = String::from("<|begin_of_text|>");
    
    // System message
    if let Some(sys) = system {
        prompt.push_str("<|start_header_id|>system<|end_header_id|>\n\n");
        prompt.push_str(sys);
        prompt.push_str("<|eot_id|>");
    }
    
    // History
    for (user, assistant) in history {
        prompt.push_str("<|start_header_id|>user<|end_header_id|>\n\n");
        prompt.push_str(user);
        prompt.push_str("<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n");
        prompt.push_str(assistant);
        prompt.push_str("<|eot_id|>");
    }
    
    // Current input
    if let Some(input) = user_input {
        prompt.push_str("<|start_header_id|>user<|end_header_id|>\n\n");
        prompt.push_str(input);
        prompt.push_str("<|eot_id|>");
    }
    
    // Prompt for assistant response
    prompt.push_str("<|start_header_id|>assistant<|end_header_id|>\n\n");
    
    prompt
}

// ═══════════════════════════════════════════════════════════════════
// OpenChat Format
// From templates.spell: Generic fallback
// ═══════════════════════════════════════════════════════════════════

fn render_openchat(
    system: Option<&str>,
    history: &[(String, String)],
    user_input: Option<&str>,
) -> String {
    let mut prompt = String::new();
    
    // System message
    if let Some(sys) = system {
        prompt.push_str("GPT4 Correct System: ");
        prompt.push_str(sys);
        prompt.push_str("<|end_of_turn|>");
    }
    
    // History
    for (user, assistant) in history {
        prompt.push_str("GPT4 Correct User: ");
        prompt.push_str(user);
        prompt.push_str("<|end_of_turn|>");
        prompt.push_str("GPT4 Correct Assistant: ");
        prompt.push_str(assistant);
        prompt.push_str("<|end_of_turn|>");
    }
    
    // Current input
    if let Some(input) = user_input {
        prompt.push_str("GPT4 Correct User: ");
        prompt.push_str(input);
        prompt.push_str("<|end_of_turn|>");
    }
    
    // Prompt for assistant response
    prompt.push_str("GPT4 Correct Assistant:");
    
    prompt
}

// ═══════════════════════════════════════════════════════════════════
// Mistral Format
// From templates.spell: Mistral/Mixtral models
// ═══════════════════════════════════════════════════════════════════

fn render_mistral(
    system: Option<&str>,
    history: &[(String, String)],
    user_input: Option<&str>,
) -> String {
    let mut prompt = String::from("<s>");
    
    // System + first user combined in Mistral format
    let system_text = system.unwrap_or("You are a helpful assistant.");
    
    // History
    for (i, (user, assistant)) in history.iter().enumerate() {
        if i == 0 {
            prompt.push_str(&format!("[INST] {} {} [/INST]", system_text, user));
        } else {
            prompt.push_str(&format!("[INST] {} [/INST]", user));
        }
        prompt.push_str(assistant);
        prompt.push_str("</s>");
    }
    
    // Current input
    if let Some(input) = user_input {
        if history.is_empty() {
            prompt.push_str(&format!("[INST] {} {} [/INST]", system_text, input));
        } else {
            prompt.push_str(&format!("[INST] {} [/INST]", input));
        }
    }
    
    prompt
}

// ═══════════════════════════════════════════════════════════════════
// Auto-Detection
// From templates.spell: detect_template_family()
// ═══════════════════════════════════════════════════════════════════

/// Auto-detect template family from model name
pub fn detect_template_family(model_name: &str) -> TemplateFamily {
    let name_lower = model_name.to_lowercase();
    
    if name_lower.contains("qwen") || name_lower.contains("chatglm") || name_lower.contains("phi") {
        TemplateFamily::ChatML
    } else if name_lower.contains("llama-3") || name_lower.contains("llama3") {
        TemplateFamily::Llama3
    } else if name_lower.contains("mistral") || name_lower.contains("mixtral") {
        TemplateFamily::Mistral
    } else {
        TemplateFamily::OpenChat
    }
}
