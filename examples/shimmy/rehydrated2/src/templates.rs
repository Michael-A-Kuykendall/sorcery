#[derive(Debug, Clone, Copy)]
pub enum TemplateFamily {
	ChatML,
	Llama3,
	OpenChat,
	Mistral,
}

impl TemplateFamily {
	pub fn render(&self, system: Option<&str>, history: &[(String, String)], user_input: Option<&str>) -> String {
		match self {
			TemplateFamily::ChatML => render_chatml(system, history, user_input),
			TemplateFamily::Llama3 => render_llama3(system, history, user_input),
			TemplateFamily::OpenChat => render_openchat(system, history, user_input),
			TemplateFamily::Mistral => render_openchat(system, history, user_input),
		}
	}

	pub fn stop_tokens(&self) -> Vec<String> {
		match self {
			TemplateFamily::ChatML => stop_tokens_chatml(),
			TemplateFamily::Llama3 => stop_tokens_llama3(),
			TemplateFamily::OpenChat => stop_tokens_openchat(),
			TemplateFamily::Mistral => stop_tokens_openchat(),
		}
	}
}

pub fn detect_template_family(model_name: &str) -> TemplateFamily {
	let n = model_name.to_ascii_lowercase();
	if n.contains("qwen") || n.contains("chatglm") || n.contains("phi") {
		return TemplateFamily::ChatML;
	}
	if n.contains("llama-3") || n.contains("llama3") {
		return TemplateFamily::Llama3;
	}
	if n.contains("mistral") || n.contains("mixtral") {
		return TemplateFamily::Mistral;
	}
	TemplateFamily::OpenChat
}

fn render_chatml(system: Option<&str>, history: &[(String, String)], user_input: Option<&str>) -> String {
	let mut out = String::new();
	if let Some(sys) = system {
		out.push_str("<|im_start|>system\n");
		out.push_str(sys);
		out.push_str("\n<|im_end|>\n");
	}

	for (user, assistant) in history {
		out.push_str("<|im_start|>user\n");
		out.push_str(user);
		out.push_str("\n<|im_end|>\n");
		out.push_str("<|im_start|>assistant\n");
		out.push_str(assistant);
		out.push_str("\n<|im_end|>\n");
	}

	if let Some(user) = user_input {
		out.push_str("<|im_start|>user\n");
		out.push_str(user);
		out.push_str("\n<|im_end|>\n");
	}
	out.push_str("<|im_start|>assistant\n");
	out
}

fn stop_tokens_chatml() -> Vec<String> {
	vec!["<|im_end|>".into(), "<|im_start|>".into()]
}

fn render_llama3(system: Option<&str>, history: &[(String, String)], user_input: Option<&str>) -> String {
	let mut out = String::new();
	out.push_str("<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n");
	if let Some(sys) = system {
		out.push_str(sys);
	}
	out.push_str("<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n");

	for (user, assistant) in history {
		out.push_str(user);
		out.push_str("<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n");
		out.push_str(assistant);
		out.push_str("<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n");
	}

	if let Some(user) = user_input {
		out.push_str(user);
	}
	out.push_str("<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n");
	out
}

fn stop_tokens_llama3() -> Vec<String> {
	vec!["<|eot_id|>".into(), "<|end_of_text|>".into()]
}

fn render_openchat(system: Option<&str>, history: &[(String, String)], user_input: Option<&str>) -> String {
	let mut out = String::new();
	if let Some(sys) = system {
		out.push_str("GPT4 Correct System: ");
		out.push_str(sys);
		out.push_str("<|end_of_turn|>\n");
	}

	for (user, assistant) in history {
		out.push_str("GPT4 Correct User: ");
		out.push_str(user);
		out.push_str("<|end_of_turn|>\n");
		out.push_str("GPT4 Correct Assistant: ");
		out.push_str(assistant);
		out.push_str("<|end_of_turn|>\n");
	}

	if let Some(user) = user_input {
		out.push_str("GPT4 Correct User: ");
		out.push_str(user);
		out.push_str("<|end_of_turn|>\n");
	}
	out.push_str("GPT4 Correct Assistant:");
	out
}

fn stop_tokens_openchat() -> Vec<String> {
	vec!["<|end_of_turn|>".into()]
}
