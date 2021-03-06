#!/usr/bin/env bats


@test "task1: Check that we have a debug output" {
    run stat "$BATS_TEST_DIRNAME/../target/debug/task1"
    [ "$status" -eq 0 ]
}

@test "task1: Output Long String: The quick ...." {
    run "$BATS_TEST_DIRNAME/../target/debug/task1" 'q' '♥ The quick brown fox jumps over the lazy dog. ♥'
    [[ "${lines[0]}" =~ "You asked me to count all 'q' in '♥ The quick brown fox jumps over the lazy dog. ♥'" ]]
    [[ "${lines[1]}" =~ "Found 1 'q' in '♥ The quick brown fox jumps over the lazy dog. ♥'" ]]
}

@test "task1: Output Short String: ababab " {
   run "$BATS_TEST_DIRNAME/../target/debug/task1" 'a' 'ababab'
   [[ "${lines[0]}" =~ "You asked me to count all 'a' in 'ababab'" ]]
   [[ "${lines[1]}" =~ "Found 3 'a' in 'ababab'" ]]
}

@test "task1: Output Error 1" {
   run "$BATS_TEST_DIRNAME/../target/debug/task1"
   [[ "${lines[0]}" =~ "not enough arguments" ]]
}

@test "task1: Output Error 2" {
   run "$BATS_TEST_DIRNAME/../target/debug/task1"
   [[ "${lines[0]}" =~ "not enough arguments" ]]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output must be exact 2 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 'a' 'b' 'c' | wc -l | xargs"
    [ "$output" = "2" ]
}

# wc output with white spaces is trimmed by xargs
@test "task1: Output must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' 'a'  | wc -l | xargs"
    [ "$output" = "1" ]
}
