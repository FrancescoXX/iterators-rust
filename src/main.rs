/*
Iterators in Rust provide a powerful and flexible way to process data efficiently 
by transforming, filtering, and aggregating elements in a collection. 

However, unlike traditional loops, Rust’s iterators are lazy, meaning they don't do any work until explicitly instructed to. 
This laziness makes iterators incredibly efficient, as they only evaluate elements when needed, 
often combining multiple transformations into a single pass over the data. 

In this lesson, we’ll dive into the core of Rust's iterator system, 
exploring how to use methods like .map(), .filter(), and .fold() to create expressive, functional code. 
*/

fn main(){
    // Iterators - example 1
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterator demonstration
    let v1 = vec![10, 20, 30];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&20));
    assert_eq!(v1_iter.next(), Some(&30));
    assert_eq!(v1_iter.next(), None);

    // Types of iterators

    /* 1. .iter()
    Purpose: Creates an iterator that borrows each element in the collection immutably.
    Ownership: The iterator yields references (&T), not owned values: the original collection is accessible and unaltered.
    Use Case: Use iter() when you want to read or inspect elements without taking ownership or modifying the collection.
     */

    let numbers = vec![1, 2, 3];

    for num in numbers.iter() {
        println!("Got: {}", num);
    }

    println!(".iter()");
    println!("{:?}", numbers);

    /* 2. iter_mut()
    Purpose: Creates an iterator that borrows each element in a collection mutably.
    Ownership: The iterator yields mutable references (&mut T), allowing you to modify the elements in place.
    Use Case: Use iter_mut when you need to change the elements of a collection.    
    */

    let mut numbers: Vec<i32> = vec![1, 2, 3];

    for num in numbers.iter_mut() {
        *num += 1;
        println!("Got: {}", num);
    }

    println!(".iter_mut()");
    println!("{:?}", numbers);

    /* 3. into_iter()
    Purpose: Consumes the collection and creates an iterator that takes ownership of each element.
    Ownership: The iterator yields owned values (T). After iteration, the original collection is no longer accessible.
    Use Case: Useful when you want to transfer ownership of the elements.
    */

    let numbers: Vec<i32> = vec![1, 2, 3];

    for num in numbers.into_iter() {
        println!("Got: {}", num);
    }

    // println!("{:?}", numbers); // Error: value borrowed here after move

    /*
        Summary Table:
  
    | Iterator     | Yields       | Ownership | Purpose                                  |
    |--------------|--------------|-----------|------------------------------------------|
    | `iter()`     | `&T`         | Borrow    | Read-only iteration                      |
    | `iter_mut()` | `&mut T`     | Mutable   | Allows in-place modification of elements |
    | `into_iter()`| `T`          | Ownership | Consumes collection for ownership        |
  
    They give fine-grained control over data ownership and mutability during iteration.
    */

    // Methods to modify or consume iterators: map(), filter(), fold()

    let numbers = [1, 2, 3, 4, 5];

    //Map
    let squares: Vec<_> = numbers
        .iter()
        .map(|&x| x * x)
        .collect();
    println!("Map - Squares {:?}", squares);

    //Filter
    let evens: Vec<_> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("Filter - Evens {:?}", evens);

    //Fold (similar to reduce)
    let sum: i32 = numbers
        .iter()
        .fold(0, |acc, &x| acc + x);

    println!("Fold - Sum: {}", sum);


    // another example with reduce
    let reduced: i32 = (1..101).reduce(|acc, e: i32| acc + e).unwrap();
    println!("Reduced: {}", reduced);

}

pub trait Iterator {
    type Item; // associated type - Item
    fn next(&mut self) -> Option<Self::Item>;
}

  /* Recap
  
  Iterators in Rust provide a powerful and flexible way to process data efficiently 
  by transforming, filtering, and aggregating elements in a collection.
  
  Rust’s iterators are lazy, meaning they don't do any work until explicitly instructed to.
  
  Iterators are fine-grained, allowing you to control data ownership and mutability during iteration.
  
  Methods like .map(), .filter(), and .fold() provide expressive ways to transform and consume iterators.
  
  
  */