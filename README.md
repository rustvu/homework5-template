# RustVU (CS 3891) - Homework 5

## Optional / warmup exercises for the week

I highly recommend the [Rustlings project](https://github.com/rust-lang/rustlings) for practicing the basic concepts we learn in this class. These completely optional, not graded/submitted exercises can help you to teach Rust programming to your "fingers".

The recommended exercises for this week:

- `generics`
- `traits`
- `conversions`

## Assignment

This assignment contains a single Rust crate focusing on generics and traits.

The library (`matrix` crate) is supposed to implement a simple generic Matrix datatype, a 2D container of numbers, which supports basic mathematical operations.

Please, read the comments in the source code to understand what is expected from the Matrix datatype. I suggest to implement the data type in the same or similar order how the test functions are defined in the crate.

I placed `// TODO` comments in the code where I expect you to add implementation code. The test code is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`. You can also run individual tests by running `cargo test <test-name>` (see the names below).

In this assignment we build a library crate, thus there is no `main()` function and you cannot *run* the crate.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.

The homework is graded by test (no partial credits are given for failed tests):

|Test          |Points|
|--------------|------|
|new           |    10|
|size          |    10|
|indexing      |    20|
|max           |    10|
|transpose     |    10|
|add           |    20|
|mul           |    20|

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.
