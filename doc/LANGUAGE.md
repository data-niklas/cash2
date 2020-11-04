- Only higher-order functions
- Executable calls (inside the path) are treated similar to functions
- **async code**
- Dynamic typing
- Datatypes:
	- String
		- first char may be a `'~'` and will be replaced by the home dir
		- (can be escaped via `"\~I really needed that char"`)
		- `"sometext"`
		- `sometext'`
		- interpolation via `${}` 
		- e.g.: `'3 + 4 = ${3+4}'`
		- "-" removes occurences of the second string
	- Integer
		- `0x`
		- `0b`
		- `0o`
		- Decimal
		- Underscores
	- Float
		- Optional leading integer, followed by `.`
		- Followed by combination of digits
		- May be followed by `e` *| `E`* and an integer
		- e.g.: `1_124.5e10`
		- Underscores
	- Boolean
		- `true` | `false` *| `True` | `False`*
	- List
		- may contain combinations of any datatypes
		- dynamic resizing
		- allow negative indices
		- Create
			- `[first, second, third      , fourth]`
		- Update
			- `somelist[0] = 3`
			- `somelist[1..3] = 5`
			- `somelist[1..3] = otherlist[2..4]`
			- append via std function(s)
		- Delete
			- via std functions
	- Dictionary
		- may contain combinations of any datatypes
		- dynamic resizing
		- string indexing
		- Create
			- `{
				"somestring": othervalue,
				stringfromvar: 42
			}`
		- Update
			- `somedict["0"] = 3`
			- `somedict[stringfromvar] = "text"`
			- `somedict::somekey = true`
			- second indexing operator: `::ident`
			- append via std function(s)
		- Delete
			- via std functions
	- Range
		- [), left-inclusive, right-exclusive (just like Rust)
		- left < right
		- Create
			- `integer..integer`
		- +/- moves the range
		- *//: scale (lower stays the same)
			
	- *Enum?*
	- Function
		- `(head)->{body}`
		- Head:
			- `var1, var2, var3 = defaultval, var4 = defaultval2`
			- optional parameters may only be after all required parameters
		- Calls:
			- `somefuncvalue(var1, var2, var3)()()`
		- May be a built-in (e.g.: `echo`)
	- Future
		- For `async` code
		- `await` may be used and returns some value after the task is done


- Operators:
	- Indexing: ltr `[]`	11
	- Function call: ltr `()`	11
	- Not: rtl `!`	10
	- Unary Plus: rtl `+`	10
	- Unary Minus: rtl `-`	10
	- Await: rtl `await`	10
	- Exponentiation: rtl `**` 9
	- Multiply: ltr `*` 8
	- Divide: ltr `/` 8
	- Modulo: ltr `%` 8
	- Add:  ltr `+` 7
	- Subtract: ltr `-` 7
	- Bit ls: ltr `<<` 6
	- Bit rs: ltr `>>` 6
	- In: ltr `in` 5
	- Lt: ltr `<` 5
	- Gt: ltr `>` 5
	- Lte: ltr `<=` 5
	- Gte: ltr `>=` 5
	- Ne: ltr `!=` 4
	- Eq: ltr `==` 4
	- And: ltr `&` 3
	- Xor: ltr `^` 2
	- Or: ltr `|` 1
	- Async: rtl `async` 0

- Control structures
	- Conditionals:
		- If:
			```
			if expr {
				somecode
			}
			elif otherexpr{

			}
			else{

			}
			```
	- Loops:
		- `break()`
		- `continue()`
		- While
			```
			while expr {
				do sth
			}
			```
		- For
			```
			for varname in ListOrRange {
				dosthwithvarname
			}
			```


- Comments
	- Line comment:
		- `# asdasdasdasd asda dasd useful note`
	
	- Multiline comments:
		```
		/* some long comment
			which ends 
			somewhere else
			*/
		```

- Piping:
	- `echo("asd / params") | lolcat()`
	- `a = $ echo("Hello ") | lolcat("asd") + "world"`
	-  Captured via `$`

Env vars:
	- `$IDENT`
	- e.g.: `$HOME`