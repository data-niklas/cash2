- Only higher-order functions
- Executable calls (inside the path) are treated similar to functions
- **async code**
- Dynamic typing
- Datatypes:
	- String
		- first char may be a `'~'` and will be replaced by the home dir
		- (can be escaped via `"\\~I really needed that char"`)
		- `"sometext"`
		- `sometext'`
		- interpolation via `${}` 
		- e.g.: `'3 + 4 = ${3+4}'`
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
	- Indexing: ltr `[]`	0
	- Function call: ltr `()`	0
	- Not: rtl `!`	1
	- Unary Plus: rtl `+`	1
	- Unary Minus: rtl `-`	1
	- Await: rtl `await`	1
	- Exponentiation: rtl `**` 2
	- Multiply: ltr `*` 3
	- Divide: ltr `/` 3
	- Modulo: ltr `%` 3
	- Plus:  ltr `+` 4
	- Minus: ltr `-` 4
	- Bit ls: ltr `<<` 5
	- Bit rs: ltr `>>` 5
	- In: ltr `in` 6
	- Lt: ltr `<` 6
	- Gt: ltr `>` 6
	- Lte: ltr `<=` 6
	- Gte: ltr `>=` 6
	- Ne: ltr `!=` 7
	- Eq: ltr `==` 7
	- And: ltr `&` 8
	- Xor: ltr `^` 9
	- Or: ltr `|` 10
	- Async: rtl `async` 11

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