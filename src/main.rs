use openai_api_rust::{Auth, Message, OpenAI, Role};
use openai_api_rust::chat::{ChatApi, ChatBody};

fn main() {
    let auth = Auth::new("KEY");
    let openai = OpenAI::new(auth, "URL");
    let body = ChatBody {
        model: "test".to_string(),
        max_tokens: Some(200),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        temperature: Some(1_f32),
        top_p: Some(1_f32),
        n: None,
        stream: Some(false),
        messages: vec![
            Message {
                role: Role::User,
                content: "What LLM are you?".to_string(),
            }
        ],
        stop: None,
        user: None,
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    println!("{message:?}");
}
