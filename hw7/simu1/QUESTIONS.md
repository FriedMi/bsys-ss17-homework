# QUESTIONS: Thread Real Posix API

In this section, we’ll write some simple multi-threaded programs and use a specific tool, called helgrind, to find problems in these programs.

Read the README in the homework download for details on how to build the programs and run helgrind.

## Questions

Do not copy paste all output of valgrind, only the parts, where your answers rely on. Run your tests on the bsys container (not your notebook!).

1. First build `main-race.c`. Examine the code so you can see the (hopefully obvious) data race in the code. Now run helgrind (by typing **valgrind --tool=helgrind ./main-race**) to see how it reports the race. Does it point to the right lines of code? What other information does it give to you?

2. Correct your code:

    2.1 What happens when you remove one of the offending lines of code?

    2.2 Now add a lock around one of the updates to the shared variable,

    2.3 and then around both.

    2.4 What does helgrind report in each (2.1, 2.2 and 2.3) of these cases?

    Check in your corrected file `main-race.`

3. Now let’s look at `main-deadlock.c`. Examine the code. This code has a problem known as deadlock (which we discuss in much more depth in a forthcoming chapter). Can you see what problem it might have? Does the program run? Explain!

4. Now run helgrind on this code. What does helgrind report? Do not copy the output into your answers, explain what helgrind tells you!

5. Now run helgrind on `main-deadlock-global.c`.

    5.1 Examine the code; does it have the same problem that `main-deadlock.c` has?

    5.2 Run the program.

    5.3 Should helgrind be reporting the same error?

    5.4 What does this tell you about tools like helgrind?

6. Let’s next look at `main-signal.c`. This code uses a variable (done) to signal that the child is done and that the parent can now continue. Why is this code inefficient? (what does the parent end up spending its time doing, particularly if the child thread takes a long time to complete?)

7. Now run helgrind on this program. What does it report? Is the code correct?

8. Now look at a slightly modified version of the code, which is found in `main-signal-cv.c`. This version uses a condition variable to do the signaling (and associated lock).

    8.1 Why is this code preferred to the previous version? Is it correctness, or performance, or both?

9. Once again run helgrind on **main-signal-cv**. Does it report any errors?
