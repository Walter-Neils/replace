# replace
A simple command line utility to allow you to replace the contents of a file you're reading from earlier in a pipe.

## The problem
If you've ever tried to do something like `cat source-file | <modify content> >source-file`, you know that `bash` opens output streams *BEFORE* running your commands, which means that `output-file` is already truncated when `cat` goes to read it. 

This utility is for that exact issue: you can just run `cat source-file | <modify content> | replace source-file` and everything works as expected.
