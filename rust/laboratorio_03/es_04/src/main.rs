use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use crate::CellId::Input;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackId(usize);


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}
struct InputCell<T> {
    value: T,
}
struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks: HashMap<usize, Box<dyn 'a + FnMut(T)>>,
    next_callback_id: usize,
}
#[derive(Default)]
pub struct Reactor<'a, T: Default> {
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
    dependencies: HashMap<CellId, HashSet<ComputeCellId>>,
}
// since our cells are going to mostly compute integers, we will allow them to contain a generic type T that implements
// copy + partialeq ( operations )
// default is used so that we don't have to set everything when creating a new struct
impl <'a, T: Copy + PartialEq + Default> Reactor<'a,T>{
    pub fn new() -> Self{
        Reactor::default()
    }

    // create a new input cell inside the app > returns its index
    pub fn create_input(&mut self,input_value: T) -> usize {
        let current_items = self.input_cells.len();
        self.input_cells.push(InputCell{value: input_value});
        current_items

    }

    /// create a compute cell
    /// if an error is encountered, the cellId is returned
    /// else the new computecellid is returned
    pub fn create_compute<F: 'a + Fn(&[T])->T>(&mut self,dependencies: &[CellId],compute_func: F)->Result<ComputeCellId,CellId>{
        let values = self.values(dependencies)?; // retrieve vec of all values to pass to compute function
        // create new compute cell
        let compute_cell = ComputeCell{
            value: compute_func(&values), // value is >> function applied to values from dependency cells
            dependencies: dependencies.to_vec(), // dependencies are parameters passed by function
            compute_func: Box::new(compute_func), // compute func is parameter passed by function
            callbacks: HashMap::new(), // default
            next_callback_id: 0, // todo
        };
        let next_id = self.compute_cells.len(); // save the index where the new compute cell is saved
        self.compute_cells.push(compute_cell); // push the new compute_cell into the compute_cells array of Reactor
        let compute_cell_id = ComputeCellId(next_id);
        for &dependency in dependencies{ // update Reactor dependencies hashMap, inserting in each of the dependencies[CellId] the new compute_cell_id
            self.dependencies
                .entry(dependency)
                .or_default()
                .insert(compute_cell_id);
        }
        Ok(compute_cell_id)
    }


    // set a new value
    // this function sets a new value for the given input cell id, returns true if the update is done correctly
    // when updating, it is also necessary to update all the consequent cells >>
    pub fn set_value(&mut self,id: InputCellId,new_value: T)->bool{
        match self.input_cells.get_mut(id.0){
            Some(input_cell) => {
                input_cell.value = new_value;
                let mut updated = HashMap::new();
                // creates a new HashMap that contains all computecells and their value >>>>
                // the goal of this is to keep track of all computecells, and update all of them.
                self.update_dependencies(&CellId::Input(id),&mut updated);
                /*
                for (id, old_value) in updated {
                    let compute_cell = self.compute_cells.get_mut(id.0).unwrap();
                    if compute_cell.value != old_value {
                        for callback in compute_cell.callbacks.values_mut() {
                            callback(compute_cell.value)
                        }
                    }
                }
                 */
                true
            }
            None => false
        }
    }

    // to create a compute func, we need the specified dependencies for that cell + compute function
    /*
    pub fn value(&self,id: CellId)->Option<T>{
        match id{
            CellId::Input(e) => {self.input_cells.get(id.0).map(|x| x.value)},
            CellId::Compute(e) => {self.compute_cells.get(id.0).map(|x| x.value)},
        }
    }

     */
    pub fn value(&self,id:CellId)->Option<T>{ // get value of cell with cellId, wether input or compute
        match id{
            CellId::Input(e) => {
                self.input_cells.get(e.0).map(|x| x.value)
            },
            CellId::Compute(e) => {
                self.compute_cells.get(e.0).map(|x| x.value)
            }
        }
    }

    // get values of all cells - if error, return CellId that has an error
    pub fn values(&self,cell_ids: &[CellId])->Result<Vec<T>,CellId>{
        cell_ids.iter()
            .map(|&id| self.value(id).ok_or(id)) // if can't retrieve value of said cell, return the id that is causing an error
            .collect()
    }




    fn update_dependencies(&mut self,changed_input_cell_id: &CellId,updated: &mut HashMap<ComputeCellId,T>){
        /// retrieve the row containing all the compute cells to update
        if let Some(compute_cell_ids) = self.dependencies.get(changed_input_cell_id){
            for compute_cell_id in compute_cell_ids.to_owned(){ // iterate over all the compute_ids found and update values accordingly
                let compute_cell = &self.compute_cells[compute_cell_id.0]; // save the current compute_cell
                let values = self.values(&compute_cell.dependencies).unwrap(); // get the values for given cell using .values() function >> retrieves all input_cells values based on this compute_cells dependencies
                let new_value = (compute_cell.compute_func)(&values); // calculate the new value according to current_compute_cell compute function

                // if the new value is not the same as the old one, update it >>
                if new_value != compute_cell.value{ // if the newfound value is not the same as the current_compute_cell, it means it's outdated >> update it
                    updated.entry(compute_cell_id).or_insert(compute_cell.value); // check if the entry in the hashmap exists, if not insert it.
                    self.compute_cells[compute_cell_id.0].value = new_value;
                    self.update_dependencies(&CellId::Compute(compute_cell_id),updated);// todo check if needed /// why is it passing compute_cell_id? it will return no value..
                }

            }
        }
    }

    pub fn add_callback<F: 'a + FnMut(T)>(&mut self, id: ComputeCellId, callback: F, ) -> Option<CallbackId> { // ads a callback to a compute cell, returns the callback id
        let compute_cell = self.compute_cells.get_mut(id.0)?;
        compute_cell.next_callback_id += 1;
        compute_cell.callbacks.insert(compute_cell.next_callback_id, Box::new(callback));
        Some(CallbackId(compute_cell.next_callback_id))
    }
}

// devo quindi creare una serie di celle
// una funzione che cambia il valore di una cella > e che aggiorna tutte le ComputeCell, chiamando la funzione al loro interno *ComputeCell.fun()*
// ->-> per aggiornarle posso o iterare ogni volta su ogni cella, controllando se l'id di quella cella è uguale
// ->->oppure fare una hashmap in cui accedo direttamente alle celle dove quell'id è presente

fn add(item: &[i32]) -> i32{
    let mut tot = 0;
    for i in 0..item.len(){
        tot += item[i]
    }
    tot
}
fn multiply(item: &[i32]) ->i32{
    let mut tot = 1;
    for i in 0..item.len(){
        tot *= item[i]
    }
    tot
}
fn main() {
    println!("Hello, world!");
    // create reactor, insert 2 new cells
    let mut my_reactor: Reactor<i32> = Reactor::new();
    let input1 = my_reactor.create_input(5); // create two input cells
    let input2 = my_reactor.create_input(10);
    let cellid1 = CellId::Input(InputCellId(input1));
    let cellid2 = CellId::Input(InputCellId(input2));
    let mycells = vec![cellid1,cellid2];
    let compute1 = my_reactor.create_compute(&mycells[0..],&add);
    let compute2 = my_reactor.create_compute(&mycells[0..],&multiply);

    println!("10 + 5  > {:?}",my_reactor.compute_cells.get(0).unwrap().value);
    println!("10 * 5  > {:?}",my_reactor.compute_cells.get(1).unwrap().value);

    let succesful = my_reactor.set_value(InputCellId(input1),50);

    println!("10 + 50 > {:?}",my_reactor.compute_cells.get(0).unwrap().value);
    println!("10 * 50 > {:?}",my_reactor.compute_cells.get(1).unwrap().value);



}
