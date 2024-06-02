use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
enum Option2<T> {
    None,
    Some(T),
}
impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
    }
    fn map<R>(&self, f: fn(&T) -> R) -> Option2<R> {
        return match self {
            Option2::None => Option2::None,
            Option2::Some(value) => Option2::Some(f(&value)),
        };
    }
}

// ----------

/* #[derive(Debug)]
enum Result2<R, E> {
    Ok(R),
    Err(E),
}
impl<R, E> From<Result2<R, E>> for Result<R, E> {
    fn from(value: Result2<R, E>) -> Self {
        match value {
            Result2::Ok(value) => Ok(value),
            Result2::Err(e) => Err(e),
        }
    }
}
fn error_me(throw: bool) -> Result2<(), usize> {
    if throw {
        return Result2::Err(42);
    }
    Result2::Ok(())
} */

fn error_me(throw: bool) -> Result<()> {
    if throw {
        return Err(anyhow!("error_me thrown"));
    }

    std::fs::read("./foo").context("undoable: ./foo read")?;
    // std::fs::read("./foo").with_context(|| "undoable")?;

    Ok(())
}

// --------

fn main() -> Result<(), usize> {
    println!("Hello, world!");

    // let foo = Some(5);
    //
    // if foo.is_some() {
    //     let value = foo.unwrap();
    // }

    let foo = Option2::Some(5);
    if foo.is_some() {
        let value = foo.map(|x| format!("{}", x));
        println!("{:?}", value);
    }

    println!("{:?}", Option2::None::<()>);

    // ----

    println!("{:?}", error_me(false));
    println!("{:?}", error_me(true));

    // let value = error_me(true)?;

    /* let error = error_me(false);
    let value: Result<(), usize> = error.into()?;

    value */
    // let value = error_me(true)?;
    // println!("{:?}", value);

    if error_me(false).is_ok() {
        println!("OK")
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn this_test() {
        assert_ne!(5, 7);
        assert_eq!(5, 7);
    }
}
