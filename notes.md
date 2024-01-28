# Notes for rust hw1
## part1
1. env 
	- cargo
		- test part${n}\_${entity\_name}\_${tested\_behavior}
2. simple functions and methonds
	- struct.member, 
	- u8 functions: overflowing\_add
	- tuple.index
	- struct { member, ... }
	- fn fname(vname: vtype) -> rtype { rvalue } // no ';' after rvalue
	- crabs
		- struct Name {
			member,
			...,}
		- memberfn(&self)
3. Crabs & Braches
	- Vec::new(), Vec<T>, Vec.len(), Vec.push(T), Vec[index]
	- need deeper learning
		- mut, &mut
		- '=' assign -> move, '= xxx.clone()' -> clone and move, '= &xxx' reference -> borrow
			- the concept of borrow is not very meaningful to immutible reference, how can many borrow one item?
			- but you can thought in this way: borrow the information of the item instead of the item itself. Maybe take a picture or 3D modle? And the immutible reference is only valid if no other one can mutate it. 
		- reference of collection
			- for i in &v means i is reference to the element i
	- control
		- if condition {} else {}
		- for element in elements {}
	- pub enum Option<T> {
		Some(T),
		None
		}

## part 2
1. references
	- can reference y = &x be moved? z = &y = &&x ?
2. collection process synthesis
	- Vec
	- Hashmap
	- Option extract, if ..., ?, match

## part 3
1. String and str
	- string and char * in c
	- &str.to\_string(), move ownership
	- equals to String::from(&str)
2. HashMap
	- get(&K) -> Option<&V>
	- get\_mut(&K) -> Option<&mut V>
3. Option
	- use option? in fn returns option or result, if None, the fn directly return None or something
	- if let Some(x) = Option(...) {
	- match enums {
		None => ..., 
		- a bit like C switch, but more powerful	
3. Result<T, E> {
	Ok(T), 
	Err(E),
4. type casting
	int\_x as f64
