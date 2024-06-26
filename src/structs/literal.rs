use super::{exec_env::ExecuteScope, Block, BlockError, ExecuteEnv};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
  Int(i64),
  String(String),
  Boolean(bool),
  Block(BlockLiteral),
  List(Vec<Literal>),
  Void,
}

impl ToString for Literal {
  fn to_string(&self) -> String {
    match self {
      Literal::Int(i) => i.to_string(),
      Literal::String(s) => s.clone(),
      Literal::Boolean(b) => b.to_string(),
      Literal::Block(b) => format!("Block {}", b.block.proc_name),
      Literal::List(list) => {
        format!(
          "[{}]",
          list
            .iter()
            .map(|l| match l {
              Literal::String(s) => format!("{s:?}"),
              _ => l.to_string(),
            })
            .collect::<Vec<String>>()
            .join(", ")
        )
      }
      Literal::Void => "<Void>".to_string(),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BlockLiteral {
  pub scopes: Vec<ExecuteScope>,
  pub block: Block,
}

impl BlockLiteral {
  pub fn execute_without_scope(
    &self,
    exec_env: &mut ExecuteEnv,
    inner_vars: impl FnOnce(&mut ExecuteEnv),
  ) -> Result<Literal, BlockError> {
    let BlockLiteral { scopes, block } = self;
    let is_closure = !scopes.is_empty();

    let freezed = exec_env.freeze_scope();
    exec_env.new_scope();
    if is_closure {
      exec_env.new_scopes(scopes.to_vec());
    }
    inner_vars(exec_env);
    let result = block.execute_without_scope(exec_env)?;
    if is_closure {
      exec_env.back_scopes();
    }
    exec_env.back_scope();
    exec_env.reload_scope(freezed);

    Ok(result)
  }
}
