pub const FISH: &str = r#"
function __doin_current_token
  commandline --current-token --cut-at-cursor
end

function __doin_tokens
  set --local tokens \
    (
      commandline \
        --current-process \
        --cut-at-cursor \
        --tokenize
    ) (__doin_current_token)
  for token in $tokens
    echo $token
  end
end

function __doin_subcommand
  test (__doin_current_token) != (__doin_tokens)[2]
end

function __doin_complete_subcommand
  set --local tokens (__doin_tokens)
  set --local dir $tokens[2]
  set --erase tokens[1..2]
  fish --command "cd $dir; and complete --do-complete '$tokens'"
end

complete --command doin \
  --condition "not __doin_subcommand" \
  --exclusive \
  --arguments "(__fish_complete_directories)"

complete --command doin \
  --condition "__doin_subcommand" \
  --exclusive \
  --arguments "(__doin_complete_subcommand)"
"#;
