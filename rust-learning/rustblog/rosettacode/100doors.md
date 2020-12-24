# 100 doors

[![Task](http://rosettacode.org/mw/images/thumb/b/ba/Rcode-button-task-crushed.png/64px-Rcode-button-task-crushed.png)](http://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")

**100 doors**  
You are encouraged to  [solve this task](http://rosettacode.org/wiki/Rosetta_Code:Solve_a_Task "Rosetta Code:Solve a Task")  according to the task description, using any language you may know.

There are 100 doors in a row that are all initially closed.

You make 100  [passes](http://rosettacode.org/wiki/Rosetta_Code:Multiple_passes "Rosetta Code:Multiple passes")  by the doors.

The first time through, visit every door and _toggle_ the door (if the door is closed, open it; if it is open, close it).

The second time, only visit every 2nd  door (door #2, #4, #6, ...), and toggle it.

The third time, visit every 3rd  door (door #3, #6, #9, ...), etc, until you only visit the 100th  door.

  

Task

Answer the question: what state are the doors in after the last pass? Which are open, which are closed?

  
**[Alternate](http://rosettacode.org/wiki/Rosetta_Code:Extra_credit "Rosetta Code:Extra credit"):**  As noted in this page's [discussion page](http://rosettacode.org/wiki/Talk:100_doors "Talk:100 doors"), the only doors that remain open are those whose numbers are perfect squares.

Opening only those doors is an [optimization](http://rosettacode.org/wiki/Rosetta_Code:Optimization "Rosetta Code:Optimization")  that may also be expressed; however, as should be obvious, this defeats the intent of comparing implementations across programming languages.


```rust
fn main() {
    let mut door_open = [false; 100];
    for pass in 1..100 {
        let mut door = pass;
        while door <= 100 {
            door_open[door - 1] = !door_open[door - 1];
            door += pass;
        }
    }
    for (i, &is_open) in door_open.iter().enumerate() {
        println!("Door {} is {}.", i + 1, if is_open {"open"} else {"closed"});
    }
}
```