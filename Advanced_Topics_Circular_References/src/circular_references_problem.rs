// We should not use RefCell as a  method to solve circular_reference problem
// Rather than that, we should redesign code pattern as database design

use std::rc::Rc;
use std::cell::RefCell;

struct Student<'a>
{
  name: String,
  courses: Vec<Rc<RefCell<Course<'a>>>>
}

impl<'a> Student<'a>
{
  fn new(name: String) -> Student<'a>
  {
    Student {name, courses: Vec::new()}
  }

  fn get_registered_courses(student: Rc<RefCell<Student<'a>>>) -> Vec<String>
  {
    student.borrow().courses.clone().iter()
      .map(|c| {c.borrow().name.clone()})
      .collect()
  }

  fn add_course(
    student: Rc<RefCell<Student<'a>>>,
    course: Rc<RefCell<Course<'a>>>
  )
  {
    student.borrow_mut().courses.push(course.clone());
    course.borrow_mut().students.push(student);
  }
}

struct Course<'a>
{
  name: String,
  students: Vec<Rc<RefCell<Student<'a>>>>
}

impl<'a> Course<'a>
{
  fn new(name: String) -> Course<'a>
  {
    Course {name, students: Vec::new()}
  }

  fn add_student(
    student: Rc<RefCell<Student<'a>>>,
    course: Rc<RefCell<Course<'a>>>
  )
  {
    course.borrow_mut().students.push(student.clone());
    student.borrow_mut().courses.push(course);
  }
}

pub fn main()
{
  let mut John = Rc::new(
    RefCell::new(
      Student::new("John".into())
    )
  );
  
  let mut rust_course = Rc::new(
    RefCell::new(
      Course::new("Math course".into())
    )
  );

  Student::add_course(John.clone(), rust_course);

  for course in Student::get_registered_courses(John.clone())
  {
    println!("{} has registered {}", John.borrow().name, course);
  }
}
