extern crate core;
use std::collections::HashMap;

fn add(items: &[i32]) -> i32{
    println!("{:?}",items);
    items.into_iter().sum()
}

fn multiply(items: &[i32]) -> i32{
    println!("{:?}",items);
    items.into_iter().product()
}


struct ComputeCell<'a,T> {
    val: i32,
    deps: Vec<i32>,
    fun: Box<dyn 'a + Fn(&[i32]) -> i32>,
    callbacks: HashMap<T, Box<dyn FnMut(T)>>
}
#[derive(Default)]
pub struct Reactor<'a> {
    input_cells: Vec<i32>,
    compute_cells: Vec<ComputeCell<'a,i32>>,
    compute_cell_ids: Vec<i32>,
}


impl<'a> Reactor<'a>{
    pub fn new() -> Self{
        Reactor::default()
    }

    // add inputcell
    // add computecell
    // set new value
    // update all values from compute cells


    // add input cell
    pub fn add_input_cell(&mut self,value: i32)->usize{
        let return_val = self.input_cells.len();
        self.input_cells.push(value);
        // update all dependencies
        return_val
    }


    // add compute cell
    pub fn add_compute_cell<F: 'a + Fn(&[i32])->i32>(&mut self,dependencies: Vec<i32>,compute_function: F)->usize{
        let return_val = self.compute_cells.len();
        let mut new_compute_cell = ComputeCell{
            val: 0,
            deps: dependencies.clone(),
            fun: Box::new(compute_function),
            callbacks: HashMap::new(),
        };

        self.compute_cell_ids.push(return_val as i32);
        self.compute_cells.push(new_compute_cell);

        self.update_cells();
        return return_val;



        /*
        /// assign values automatically by updating cells
        let my_values = self.get_values(dependencies);
        let mut final_value = 0;
        if my_values.ok().is_some(){
            final_value = compute_function(&my_values.unwrap());
            new_compute_cell.val = final_value;
            return return_val as i32;
        }
         */
    }

    // set new value
    pub fn update_value(&mut self,index_input_cell: usize,new_val:i32)->bool{
        // check if index is right
        if index_input_cell >= self.input_cells.len(){
            return false;
        }
        // update value, then call function to update all dependencies
        if let x = self.input_cells.get(index_input_cell).is_some(){
            self.input_cells[index_input_cell] = new_val;
            // call to update all values of computecells;
            self.update_cells();
            return true;
        }
        false
    }


    // update all compute cells
    pub fn update_cells(&mut self) {
        // update all cells
        // iterate over all compute cells and execute their compute function, passing them their dependencies.
        for i in &self.compute_cell_ids{
            // get dependencies, get values from input cells, apply compute_cell function to values
            let dependencies = self.compute_cells[*i as usize].deps.clone();
            let values = self.get_values(dependencies).unwrap();
            let new_value:i32 = (self.compute_cells[*i as usize].fun)(&*values);
            if new_value != self.compute_cells[*i as usize].val{
                self.compute_cells[*i as usize].val = new_value;
            }
        }

        println!("Updated all values");
    }






    pub fn get_value(&self,input_cell_id: usize) -> Option<i32>{
        if input_cell_id < self.input_cells.len(){
            return self.input_cells.get(input_cell_id).map(|x| *x)
        }else{
            None
        }
    }

    pub fn get_values(&self,input_cell_ids: Vec<i32>)->Result<Vec<i32>,i32>{
        input_cell_ids.iter()
            .map(|id| self.get_value(*id as usize).ok_or(-1)) // if can't retrieve value of said cell, return the id that is causing an error
            .collect()
    }


}

fn main() {
    println!("Hello, world!");

    let mut my_reactor = Reactor::new();
    my_reactor.add_input_cell(10);
    my_reactor.add_input_cell(20);

    println!("Input cell 1 > {}",my_reactor.input_cells[0]);
    println!("Input cell 2 > {}",my_reactor.input_cells[1]);



    let compute1 = my_reactor.add_compute_cell(vec![0,1],&add);
    let compute2 = my_reactor.add_compute_cell(vec![0,1],&multiply);

    println!("Value of compute 1 > 10 + 20 = {} ",my_reactor.compute_cells[compute1].val);
    println!("Value of compute 1 > 10 * 20 = {} ",my_reactor.compute_cells[compute2].val);

}
