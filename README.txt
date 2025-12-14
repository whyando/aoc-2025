
Day 1: Secret Entrance
    Modulo arithmetic

    Part 1 is simple modulo counting.
    For part 2, we partion the rotations into sections of 100 using div
    for example: 220 div 100 = section 2
    680 div 100 = section 6
    so if we moved from 220 to 680 then we crossed 0 `6-2` times

    When we land onto the boundary we want that to count as moving into a new section. So if we're going forward then
    the sections as 0-99, 100-199, 200-299 are fine. But if we are going backward then we want to use the sections
    1-100,101-200,201-300, ..etc


Day 2: Gift Shop
    Inclusion-Exclusion, Summation formula

    For part 1, we observe that the invalid IDs can only be of even length, and must take the form with a string of
    repeated digits repeated twice eg 'xyzxyz'. So for some partition where both the start and end have the same
    decimal length N (divisible by 2) we can easily find all the invalid IDs by considering the corrosponding interval
    over integers of decimal length N/2 where we chop the later half off the start and end.

    eg if we're counting 443153-745857, we look at 443-745. We need to check the start and end seperately, so
    443443 is not in, 745745 is in. But then the neat part is that everything in between is in, so we can use the
    summation formula to get the sum we want: sum from 444 to 744 inclusive, (multiplied by 1001)


    For part 2, its the same except now the divisor isn't just 2, so we can do identical summation type counting.
    However, we have an issue with double counting eg 111111 is counted multiple times:
    1 six times, 11 three times, 111 twice.
    We can use inclusion exclusion principle to sort this out:
    let A = 1-repeats
    let B = 2-repeats
    let C = 3-repeats

    sum(A union B union C) = sum(A) + sum(B) + sum(C) - sum(A and B) - sum(A and C) - sum(B and C) + sum(A and B and C)

    However, if we look closer at a set like `A and C` if a number is a 1-repeat AND a 3-repeat, that's just the same
    as the 1-repeat set so it reduces to A.
    Likewise, the 2-repeat and 3-repeat overlap is again just A.

    Our sum simplifies to:
    sum(A union B union C) = sum(A) + sum(B) + sum(C) - sum(A) - sum(A) - sum(A) + sum(A)
                        = sum(B) + sum(C) - sum(A)
                        = sum(2-repeats) + sum(3-repeats) - sum(1-repeats)

Day 3: Lobby
    Greedy algorithm

    For part 1, the largest number will always start with the largest digit present - except if the digit appears at the very end.
    So we just need to find the find instance of the largest digit in x[0..x.len()-1], and after that the first instance of the
    largest digit in what remains.
    
    In part 2, its the same greedy type thinking generalised: for every digit iterate over the range for the first occurence of max digit. If the number has N decimal digits, and we're looking for 12 digits this approach has complexity O(N^12)

    I also designed a solution linear in N which only reads each digit once, into a data structure design to track the largest digits
    seen so far, but also to flush preceeding digits if a larger digit comes along.
    I'm not quite sure why but this was slower in practice

Day 4: Printing Department
    Greedy algorithm

    Part 1 is just simple counting.
    For part 2, we can just remove rolls greedily in any order.
    To optimise this I kept count of the number of rolls adjacent to every square. When removing a roll we can then update the
    counts for the adjacent squares and check at the same time whether they hit the threshold to be removed.

    This is the first problem I used any unsafe rust. The bulk of the loops were a lot of array accesses so it felt like it
    would benefit.

Day 5: Cafeteria
    Intervals

    If we sort the intervals by the start value, then counting the size of the union is very easy since we can track the largest
    value we already added, and then cut that part out of the next range.
    Next time at least get us to sum up the fresh ingredents instead of counting them.

Day 6: Trash Compactor
    Just some fiddly parsing. The operators in the final line were always left aligned so it was easy to extract the
    relevant columns.

Day 7 Laboratories
    We can easily track the number of rays that reach each position.

Day 8: Playground
    So my thinking on this problem was that it is just Kruskal's algorithm, except kruskals is really slow in a dense graph
    where you can move between any pair of edges. So I did part 1 with kruskals and then part 2 with Prim's algorithm which
    finds the equivalent minimum spanning tree, which does a lot better on a sparse graph.

    I used linked lists for the labels, so I could relabel a huge amount of nodes very quickly. As long as you only relabel
    the smaller set in a merge I *think* this is amortized constant time.

    Apparently this is normally done with a disjoint-set forest also known as union find.

    Day 8 is the first day where the runtime could be improved by a lot. My solution is 7.0ms here compared to mattcl/rust soln
    of 2.4ms. Two areas of improvement: use union find technique over the linked list which is much more efficient. And also the
    graph can be simplified a lot by making assumptions about the input because you can safely assume the MST won't contain edges
    that are far apart. We can use kdtree or similar to index the nodes in 3d space and avoid any kind of N^2 iterations.
    
Day 9: Movie Theater
    This was the most disappointing problem by far for me. My first solution to this problem was a kind of flood fill algorithm, and
    naively checking each point in the full area which did get the star. I wanted to switch to a more elegant solution which checked
    the potential rectanges by using some edge intersection calculations. I hit a lot of headaches with this because there are a lot
    of complicated edge cases, my solution to this was making the shape very slightly larger, and then it was easy to calculate the
    projection lines from each vertex. This general solution worked great and was very efficient.

    Turns out the inputs had a general form of a circle with a slice out the middle, so it becomes easy to see that the solution
    will always take a very specific form which cuts out almost all the requirement to do any of the above. I think you could devise
    an extremely fast runtime solution that works on all official inputs without much trouble.

Day 10: Factory
    For part 1 you could think of this as a graph where you need to find the shortest path between 2 locations but I just checked
    every subsets of buttons with bitwise encoding and xor. which seemed good enough

    For part 2, it became clear that any kind of shortest distance algorithm was not going to work which leaves us with a linear
    programming problem to solve. I wasn't too thrilled about this, I could have probably done something like gaussian elimination
    from the top of my head but I didn't really know how it would work out with the integer-only solutions and how to deal with the
    fact that is was a maximisation problem and there was more than a single solution. I had cursor write me an LP solver using
    simplex but actually this failed because the non-integer solutions it produced weren't sufficient. Glad that I didn't waste my
    own time implementating simplex I then turned to an LP rust crate to solve the problem for the integer case which worked fine.

    This problem was a double disappointment: I needed an external crate and also the pure rust crate I picked had poor performance.

    This is the second problem I had significant performance loss. I could either switch to a faster crate which would take no time at all
    but feels like hella cheating, or implement branch and bound myself.

Day 11: Reactor
    We say f[x] = no. of paths from x to out
    then f[x] = sum of f[y], summing over all nodes y with edges leading into x
    and f["you"] = 1

    I was feeling lazy at this point and just solved this with memoization. I might come back and redo this at some point 
    which topological ordering which would be more efficient. I also used a lot of String and HashMap stuff which should
    be replaced by numbering the nodes and Vecs instead.

    Part 2 is just the same except there are 2 different routes which are made of 3 sections each.

Day 12: 
    It takes only 10 seconds of looking at this problem to see that solving it in the general case isn't possible (assuming we want our
    program to finish execution before the heat death of the universe). So yeah the input basically only has cases that can we solved
    using trivial bounds. Starting to dislike these problems that rely on the input taking a specific pattern.
