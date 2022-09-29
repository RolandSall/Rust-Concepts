# Tuples & Strings

# Strings 

String is datatype stored on heap (just like Vec ), and you have access to that location. &str is a slice type. That means it is just reference to an already present String somewhere in the heap. &str doesn't do any allocation at runtime.

