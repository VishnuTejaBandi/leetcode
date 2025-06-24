#!/usr/bin/env fish

# Script to create new Rust problem modules
# Usage: ./new-problem.fish <problem_number> <function_name>

function main
    if test (count $argv) -ne 2
        echo "Usage: ./new-problem.fish <problem_number> <function_name>"
        echo "Example: ./new-problem.fish 218 two_sum"
        exit 1
    end

    # Get the directory where this script is located (project root)
    set PROJECT_ROOT (dirname (status --current-filename))
    echo "📁 Using project root: $PROJECT_ROOT"

    set PROBLEM_NO $argv[1]
    set FUNCTION_NAME $argv[2]
    set PROBLEM_FILE "n_$PROBLEM_NO.rs"

    # Create the problem file
    echo "pub fn $FUNCTION_NAME() {
    // TODO: Implement solution for problem $PROBLEM_NO
    todo!(\"Implement $FUNCTION_NAME\")
}" >"$PROJECT_ROOT/src/problems/$PROBLEM_FILE"

    # Create the test file
    echo "#[cfg(test)]
mod tests {
    use crate::problems::n_$PROBLEM_NO::$FUNCTION_NAME;

    #[test]
    fn test_$FUNCTION_NAME() {
        // TODO: Add comprehensive test cases
        // assert_eq!($FUNCTION_NAME(), expected_result);
    }
}" >"$PROJECT_ROOT/src/tests/$PROBLEM_FILE"

    # Update problems/mod.rs
    set PROBLEMS_MOD "$PROJECT_ROOT/src/problems/mod.rs"
    if not grep -q "pub mod n_$PROBLEM_NO;" "$PROBLEMS_MOD"
        echo "pub mod n_$PROBLEM_NO;" >>"$PROBLEMS_MOD"
        echo "Added module declaration to problems/mod.rs"
    end

    # Update tests/mod.rs
    set TESTS_MOD "$PROJECT_ROOT/src/tests/mod.rs"
    if not grep -q "mod n_$PROBLEM_NO;" "$TESTS_MOD"
        echo "mod n_$PROBLEM_NO;" >>"$TESTS_MOD"
        echo "Added module declaration to tests/mod.rs"
    end

    # Update lib.rs to include modules if they don't exist
    set LIB_RS "$PROJECT_ROOT/src/lib.rs"
    if not grep -q "pub mod problems;" "$LIB_RS"
        echo "pub mod problems;" >>"$LIB_RS"
    end

    if not grep -q "pub mod tests;" "$LIB_RS"
        echo "pub mod tests;" >>"$LIB_RS"
    end

    echo "✅ Created problem files:"
    echo "   📁 $PROJECT_ROOT/src/problems/$PROBLEM_FILE"
    echo "   📁 $PROJECT_ROOT/src/tests/$PROBLEM_FILE"
    echo "✅ Updated module declarations"
    echo ""
    echo "🚀 You can now implement your solution in $PROJECT_ROOT/src/problems/$PROBLEM_FILE"
    echo "🧪 Run tests with: cargo test $FUNCTION_NAME"
end

main $argv
