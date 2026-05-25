# F4NA Code Analysis Report
Generated with Anthropic Claude Haiku 4.5 via GitHub Copilot

## 1. FEATURES TO ADD

### Language Features
- **Comments** - Support for single-line (`//`) and/or multi-line (`/* */`) comments
- **For Loops** - Already in grammar but not implemented in parser/evaluator
  - Syntax: `for` (init; condition; increment) { body }
  - Or: `for` (x in array) { body } (iterator style)
- **Break and Continue** - Flow control for loops
- **Switch/Match Statements** - For multi-way branching
- **Do-While Loops** - Post-test loops
- **String Interpolation** - Instead of requiring `"text" + var + "more"`, support `"text {var} more"`

### Type System
- **Type Annotations** - Optional static typing: `let x: int = 5;`
- **Custom Types/Structs** - Already in TODO.md
  - Simple struct definitions: `type Point { x: int, y: int }`
  - Access via dot notation: `p.x`
- **Type Casting** - Functions like `int()`, `float()`, `string()` to convert between types

### Array/Collection Operations
- **Array Methods** - Built-in methods for array operations:
  - `.length()` or `.len()` - Get array length (currently need stdlib function)
  - `.push()` / `.append()` - Add element
  - `.pop()` - Remove and return last element
  - `.slice(start, end)` - Get subarray
  - `.map()`, `.filter()`, `.reduce()` - Functional operations
- **Destructuring** - `let {a, b} = array;` or `let {x, y} = point;`

### Standard Library Functions (Possibly)
- **String Operations**: `substring()`, `uppercase()`, `lowercase()`, `trim()`, `split()`, `join()`
- **Array Operations**: `reverse()`, `sort()`, `shuffle()`
- **Math**: `min()`, `max()`, `round()`, `ceil()`, `floor()`, `pow()` (in addition to ** operator)
- **Type Checking**: Runtime type predicates `is_int()`, `is_string()`, etc.
- **File I/O**: Higher-level file functions beyond syscalls
- **String Formatting**: `format()` or printf-style formatting

### Module/Import System
- **Better Module Support** - Current import system exists but may need:
  - Namespacing
  - Import specific items: `import foo from bar;`
  - Relative imports

### Error Handling
- **Try-Catch or Error Propagation** - Currently errors just crash
- **Custom Error Types**

---

## 2. BUILT-IN FUNCTIONS TO ADD

### Math Functions (Currently Missing)
- `ceil(x)` - Round up to nearest integer
- `floor(x)` - Round down to nearest integer
- `round(x)` - Round to nearest integer
- `min(a, b)` - Minimum of two numbers
- `max(a, b)` - Maximum of two numbers
- `pow(x, y)` - Power (alternative to `**` operator)
- `exp(x)` - e^x
- `ln(x)` - Natural logarithm
- `asin(x)`, `acos(x)`, `atan(x)` - Inverse trig functions
- `sign(x)` - Return -1, 0, or 1

### String Functions (Currently Missing)
- `uppercase(s)` / `upper(s)` - Convert to uppercase
- `lowercase(s)` / `lower(s)` - Convert to lowercase
- `substring(s, start, end)` / `substr(s, start, len)` - Extract substring
- `index_of(s, substr)` / `find(s, substr)` - Find substring position
- `trim(s)` - Remove leading/trailing whitespace
- `ltrim(s)`, `rtrim(s)` - Remove leading/trailing whitespace on one side
- `split(s, delimiter)` - Split string into array
- `join(array, delimiter)` - Join array into string
- `replace(s, old, new)` - Replace all occurrences
- `startswith(s, prefix)` / `endswith(s, suffix)` - String tests
- `repeat(s, count)` - Repeat string N times

### Array Functions (Currently Missing)
- `len(array)` / `length(array)` - Get array length
- `push(array, value)` / `append(array, value)` - Add element (note: current `+` operator modifies)
- `pop(array)` - Remove and return last element
- `shift(array)` - Remove and return first element
- `unshift(array, value)` - Add to front
- `reverse(array)` - Reverse array
- `sort(array)` - Sort array
- `contains(array, value)` - Check membership
- `index_of(array, value)` - Find element position
- `slice(array, start, end)` - Extract subarray
- `join(array, separator)` - Join into string (duplicate with string functions)

### Type Functions (Partially Exists)
- `typeof(x)` ✓ (exists)
- `int(x)` - Convert to integer
- `float(x)` - Convert to float
- `string(x)` - Convert to string
- `bool(x)` - Convert to boolean
- `is_int(x)`, `is_float(x)`, `is_string(x)`, `is_array(x)`, `is_bool(x)` - Type checks

### I/O Functions (Partially Exists)
- `print(...)` ✓ (exists)
- `println(...)` ✓ (exists)
- `input(prompt)` ✓ (exists)
- `readline()` - Read line from stdin without prompt
- `eprint(...)`, `eprintln(...)` - Print to stderr

### System Functions (Partially Exists)
- `platformid()` ✓ (exists)
- `syscall(...)` ✓ (exists)
- `exit(code)` - Exit program (users can build via syscall)
- `time()` / `now()` - Current time
- `sleep(ms)` - Sleep for milliseconds
- `random()` / `rand()` - Random number
- `seed(n)` - Seed random number generator

### Debugging Functions
- `debug(x)` / `dbg(x)` - Print debug representation
- `assert(condition, message)` - Runtime assertion

---

## 3. POTENTIAL BUGS

### Critical Bugs
1. **String Comparison Error Messages Are Confusing**
   - Lines 200-202 in `value.rs`: Trying to compare string with non-string throws error
   - But the error happens in the value comparison, not during assignment
   - **Impact**: Error types could be unclear to users

2. **No Input Validation on strlen**
   - Line 26-31 in `build_env.rs`: `c_strlen` returns 0 for non-string types
   - **Impact**: Silently fails instead of erroring; could cause subtle bugs
   - **Possible Fix**: Return error or panic on type mismatch

### Minor Issues
10. **No Tail Call Optimization** - Recursive functions could blow the stack
11. **Variables Not Scoped** - Function parameters might shadow outer variables incorrectly
    - Line 81-82 in `eval_expr.rs`: Function saves and restores `env.vars`
    - **Could be intentional**, but could also be a scoping bug

12. **No Null/None Type** - Can cause confusion about default values
    - Line 261 in `value.rs`: Non-numeric types return 0 in `as_int()`

13. **Floating Point Not Equal Check**
    - Line 15 in `eval_expr.rs`: `Token::Eq => Ok(Value::Bool(lv == rv))`
    - For floats, this uses exact equality which is fragile with FP precision
    - **Impact**: `0.1 + 0.2 == 0.3` could fail

14. **No Break/Continue in While Loops**
    - Users must use flags or other workarounds to break early

15. **Silent Fallthrough in Error Cases**
    - Some error paths might not be handled; the `panic!` statements in various places
    - **Impact**: Parser errors result in panics instead of graceful error messages
