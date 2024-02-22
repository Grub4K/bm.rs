use indoc::indoc;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    Powershell,
    Posix,
    Bash,
}

pub fn exec(shell: Shell, alias: Option<String>) -> Result<(), i32> {
    print!(
        "{}",
        match shell {
            Shell::Bash => indoc! {r#"
                # Init script for bash shells
                # To enable, run the following:
                # eval "$(command bm init bash)"

                __bm_is_cd() {
                    [[ "${1}" =~ ^(-c|--config)$ ]] && shift 2
                    [[ "${1}" == "cd" ]]
                }

                __bm_wrapper() {
                    if __bm_is_cd "${@}"; then
                        local __bm_result

                        __bm_result="$(command bm "${@}")" || return
                        cd "${__bm_result}" || return 1
                    else
                        command bm "${@}"
                    fi
                }

                alias <{alias}>=__bm_wrapper
                "#},
            Shell::Posix => indoc! {r#"
                # Init script for posix shells
                # To enable, run the following:
                # eval "$(command bm init posix)"

                __bm_is_cd() {
                    if [ "${1}" = "-c" ] || [ "${1}" = "--config" ]; then shift 2; fi
                    [ "${1}" = "cd" ]
                }

                __bm_wrapper() {
                    if __bm_is_cd "${@}"; then
                        __bm_result="$(command bm "${@}")" || return
                        cd "${__bm_result}" || return 1
                    else
                        command bm "${@}"
                    fi
                }

                alias <{alias}>=__bm_wrapper
                "#},
            Shell::Powershell => indoc! {r#"
                # Init script for powershell shells
                # To enable, run the following:
                # Invoke-Expression (& { (& bm.exe init powershell | Out-String) })

                function global:__bm_wrapper {
                    function __bm_is_cd($opts) {
                        if ($opts[0] -match '^(-c|--config)') {
                            return ($opts[2] -eq 'cd')
                        }
                        return ($opts[0] -eq 'cd')
                    }

                    if (__bm_is_cd $Args) {
                        $result = & bm.exe @args
                        if ($LASTEXITCODE -eq 0) {
                            Set-Location -Path $result
                        }
                    } else {
                        & bm.exe @args
                    }
                }

                Set-Alias -Name <{alias}> -Value __bm_wrapper -Option AllScope -Scope Global -Force
                "#},
        }
        .replace("<{alias}>", alias.unwrap_or("bm".into()).as_str())
    );
    Ok(())
}
