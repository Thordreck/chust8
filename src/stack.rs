
use arrayvec::ArrayVec;

pub const STACK_SIZE : usize = 64;

type InternalStorage = ArrayVec<[u8; STACK_SIZE]>;

struct Stack
{
    data: InternalStorage,
}

impl Stack
{
    fn new() -> Self
    {
        Stack { data: InternalStorage::new() }
    }

    fn push(&mut self, value : u8) -> Result<(), String>
    {
        if self.data.is_full()
        {
            return Err(String::from("Stack full. Cannot push"));
        }

        self.data.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<u8, String>
    {
        let value = self.data.pop();

        if value.is_none()
        {
            return Err(String::from("Stack empty. Cannot pop"));
        }

        Ok(value.unwrap())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn push_and_pop() -> Result<(), String>
    {
        let mut stack = Stack::new();

        // Cannot pop an empty stack
        assert!(stack.pop().is_err());

        stack.push(1)?;
        stack.push(2)?;
        stack.push(3)?;

        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);

        // Cannot pop an empty stack
        assert!(stack.pop().is_err());

        // Test filling up the stack
        for x in 1..=STACK_SIZE
        {
            stack.push(x as u8)?;
        }

        // We shouldn't be able to push anymore
        assert!(stack.push(0).is_err());

        // Empty the stack
        for _ in 1..=STACK_SIZE
        {
            stack.pop()?;
        }

        // Cannot pop an empty stack
        assert!(stack.pop().is_err());
        Ok(())
    }
}
