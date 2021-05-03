use easy_repl::{Repl, CommandStatus, command, args_validator};
use anyhow::{self, Context};

fn matryoshka(name: String) -> anyhow::Result<Repl<'static>> {
    let prompt = format!("{}> ", name);

    let cloned_prompt = prompt.clone();  // need to move it into closure
    let new = command! {
        "Enter new repl",
        String:name => |(name, ): (String, )| {
            let name = cloned_prompt.clone() + &name;
            let mut repl = matryoshka(name).unwrap();
            repl.run();
            CommandStatus::Done
        }
    };

    let repl = Repl::builder()
        .prompt(prompt)
        .add("new", new)
        .build()?;

    Ok(repl)
}

fn main() -> anyhow::Result<()> {
    let mut repl = matryoshka("".into())?;
    repl.run();
    Ok(())
}
