use std::{
    str::CharIndices,
    collections::HashSet,
};

use super::{
    token::Token,
    variable::Variable,
};


/*----------------------------------------------------------------------------*/
pub struct Parser<'a, 'b>
{
    source: &'a str,
    cursor: usize,
    is_in_escape: bool,
    is_in_variable: bool,
    indices_and_characters: CharIndices<'a>,
    variable_names: HashSet<&'b str>,
}


/*----------------------------------------------------------------------------*/
impl<'a, 'b> Parser<'a, 'b>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(source: &'a str,
               variable_names: HashSet<&'b str>) -> Self
    {
        Self
        {
            cursor: 0,
            is_in_escape: false,
            is_in_variable: false,
            indices_and_characters: source.char_indices(),
            source,
            variable_names,
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn value_token(&mut self, exclusive_end: usize) -> Option<Token<'a>>
    {
        if self.cursor < exclusive_end
        {
            let chunk = &self.source[self.cursor..exclusive_end];
            self.cursor = exclusive_end + 1;
            Some(Token::Value(chunk))
        }
        else
        {
            None
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn variable_name(&self, begin: usize,
                            end: usize) -> crate::Result<&'a str>
    {
        let variable_name = &self.source[begin..end];
        if self.variable_names.contains(variable_name)
        {
            Ok(variable_name)
        }
        else
        {
            return Err(format!("Unknown variable: `{}`", variable_name).into())
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn variable_token(&mut self) -> crate::Result<Option<Token<'a>>>
    {
        let mut is_in_specifier = false;
        let mut variable_name = None;
        let mut specifier = None;
        while let Some((i, character)) = self.indices_and_characters.next()
        {
            match character
            {
                ':' =>
                {
                    let name = self.variable_name(self.cursor, i)?;
                    variable_name = Some(name);
                    is_in_specifier = true;
                    self.cursor = i + 1;
                },
                '}' =>
                {
                    if is_in_specifier
                    {
                        is_in_specifier = false;
                        specifier = Some(&self.source[self.cursor..i]);
                    }
                    else
                    {
                        let name = self.variable_name(self.cursor, i)?;
                        variable_name = Some(name);
                    }

                    self.cursor = i + 1;
                    break;
                },
                _ => ()
            }
        }

        if is_in_specifier
        {
            return Err("Incomplete variable expression".into());
        }

        let variable =
            variable_name.map_or_else(
                || Err("Variable name missing".into()),
                |name| Variable::new(name, specifier))?;

        Ok(Some(Token::Variable(variable)))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn token(&mut self) -> crate::Result<Option<Token<'a>>>
    {
        if self.is_in_variable
        {
            self.is_in_variable = false;
            return self.variable_token();
        }

        while let Some((i, character)) = self.indices_and_characters.next()
        {
            match character
            {
                '@' =>
                    /* If an `@` sign preceeds the current one, that means this
                       is an escape, so starting from the cursor until this sign
                       everything should be recorded as a chunk */
                    if self.is_in_escape
                    {
                        self.is_in_escape = false;
                        let token = self.value_token(i);
                        debug_assert!(token.is_some(),
                                      "At this point there should be at least \
                                       a single `@` sign in a `Token::Value`");
                        return Ok(token);
                    }
                    /* This `@` sign might be the beginning of an escape
                       sequence */
                    else
                    {
                        self.is_in_escape = true;
                    },
                /* If an `@` sign preceeds this opening brace that means this is
                   the opening of a variable expression */
                '{' if self.is_in_escape =>
                {
                    self.is_in_escape = false;
                    return
                        match self.value_token(i - 1)
                        {
                            token @ Some(_) =>
                            {
                                self.is_in_variable = true;
                                self.cursor = i + 1;
                                Ok(token)
                            },
                            None =>
                            {
                                self.cursor = i + 1;
                                self.variable_token()
                            },
                        };
                },
                _ => (),
            }
        }

        if self.is_in_escape
        {
            Err("EOL: Because the last character is an `@` it is expected to \
                 be followed by either an `@` sign or a `{` sign followed by \
                 a variable expression".into())
        }
        else
        {
            Ok(self.value_token(self.source.len()))
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a, 'b> Iterator for Parser<'a, 'b>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    type Item = crate::Result<Token<'a>>;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn next(&mut self) -> Option<Self::Item>
    {
        self.token().transpose()
    }
}
