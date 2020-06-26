# ABSTRACT

The "backref-pattern-language" should be completely rethought.  Instead of
having the extra `--transform` flag, which is global and only applied after the
construction of the `--target`, the transformations should be applied at the
group level at the same time as the target construction.  Unfortunately, this
would mean we can't use `Regex::replace_all` anymore, but on the plus side
this would also mean we have more control and flexibility and we could even have
better performance this way.


# DETAILS

The "language" should have the following simple syntax:

```text
\{
    \s*
    (
        (?P<internal>
            \$\w+
        )
        |
        (?P<user-index>
            \d+
        )
        |
        (?P<user-name>
            [a-zA-Z_]\w*
        )
    )
    \s*
    (?P<instructions>
        :
        \s*
        (?P<format>
            fmt\(
                \s*
                (?P<fill>
                    ".*"
                )
                \s*
                ,
                \s*
                (?P<align>
                    <|\^|>
                )
                \s*
                ,
                \s*
                (?P<width>
                    \d+
                )
                \s*
                ,?
                \s*
            \)
        )
        |
        (?P<cast>
            cast\(int\)
        )
        |
        (?P<replace>
            sub\(
                \s*
                (?P<from>
                    .*?
                )
                \s*
                ,
                \s*
                (?P<to>
                    .*?
                )
                \s*
                ,?
                \s*
            \)
        )
        |
        (?P<transform>
            to\(
                (upper|lower|title|capital|correct)
            \)
        )
        (
            \s*
            ->
            \s*
            (
                (?P<format>
                    fmt\(
                        \s*
                        (?P<fill>
                            ".*"
                        )
                        \s*
                        ,
                        \s*
                        (?P<align>
                            <|\^|>
                        )
                        \s*
                        ,
                        \s*
                        (?P<width>
                            \d+
                        )
                        \s*
                        ,?
                        \s*
                    \)
                )
                |
                (?P<cast>
                    cast\(int\)
                )
                |
                (?P<replace>
                    sub\(
                        \s*
                        (?P<from>
                            .*?
                        )
                        \s*
                        ,
                        \s*
                        (?P<to>
                            .*?
                        )
                        \s*
                        ,?
                        \s*
                    \)
                )
                |
                (?P<transform>
                    to\(
                        (upper|lower|title|capital|correct)
                    \)
                )
            )
        )*
    )
    \s*
\}
```

If `{` or `}` needs to be escaped, they should be doubled, e.g. `{{` or `}}`.

Internally defined references should use the `$` prefix, e.g.

`{$index}`

User defined ones must be without:

`{1}` or `{name}`

Where the index `0` means the entire match of `--source`.

An optional "instructions" part could follow the reference, which could contain
multiple instructions, separated by `->` which also indicated the actual order
the instructions are going to be applied to the matched value, e.g.

`{ ref : fmt("_", left, 2) -> to(upper) -> to(correct) }`

The same syntax should be applied to all other flags as well, in which case
those patterns could apply the instructions themselves before the result is
used, e.g.

`--order-by {1:cast(int)}`


# EXAMPLES

```bash
$ ls -l
1-hello.flac
2-world.flac
10-stuff_and_more_stuff.flac

$ rerename -s '(\d+)-(\w+).(flac)'
           -t '{$index} - {1:sub("_", " ") -> to(title)}.{2}'
           -o {1:cast(int)}
           *.flac
1-hello.flac -> 1 - Hello.flac
2-world.flac -> 2 - World.flac
10-stuff_and_more_stuff.flac -> 3 - Stuff And More Stuff.flac
```

# FOOTNOTES

1. In the above examples `correct` means spell checked, which could be a feature
   added later on.  This feature could also be interactive: whenever in doubt,
   `rerename` should ask the user to choose from the suggestions
