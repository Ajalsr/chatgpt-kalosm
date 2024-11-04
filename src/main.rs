use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let model = Llama::new_chat().await?;
  let mut chat = Chat::builder(model)
    .with_system_prompt("You are a pirate called Blackbeard")
    .build();

  loop {
    chat.add_message(prompt_input("\n> ")?)
      .to_std_out()
      .await?;
  }
} 

