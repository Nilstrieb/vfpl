use crate::error::Span;
use crate::interpret_ast::{
    Env, FnImpl, IResult, InterpreterError, Interrupt, RuntimeFn, Value, Vm,
};
use crate::parse::ast::TyKind;
use std::cell::RefCell;
use std::rc::Rc;

fn ident(str: &str) -> Rc<str> {
    str.to_string().into()
}

impl Vm {
    pub fn add_global_functions(&mut self) {
        let mut env = (*self.current_env).borrow_mut();
        let vars = &mut env.vars;

        vars.insert(ident("println"), println());
    }
}

//////// Native functions

///
/// print a value to stdout
/// Takes a single arg of any type, called "value"
fn println() -> Value {
    Value::Fn(Rc::new(RefCell::new(RuntimeFn {
        params: vec![(ident("x"), TyKind::Any)],
        ret_ty: TyKind::Absent,
        body: FnImpl::Native(println_impl),
        captured_env: Rc::new(RefCell::new(Env::default())),
    })))
}

fn println_impl(vm: &mut Vm) -> IResult {
    let env = (*vm.current_env).borrow_mut();

    let value = env
        .get_value("x")
        .unwrap_or_else(|| unreachable!("did not find function parameter"));

    let mut stdout_lock = vm.stdout.borrow_mut();

    writeln!(stdout_lock, "{}", value).map_err(|err| {
        Interrupt::Error(InterpreterError {
            span: Span::dummy(),
            message: format!("Failed to write to stdout: {}", err),
        })
    })?;

    Err(Interrupt::Return(Value::Absent))
}
