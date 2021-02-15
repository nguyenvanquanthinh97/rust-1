// Instead of using Rc and Reffcell to handle circular_reference_problem
// We should rather redesign code pattern
// If we can't, it's ok to use Rc and Reffcell to handle

struct Student
{
  name: String
}

impl Student
{
  fn new(name: String) -> Student
  {
    Student{name}
  }

  fn get_registered_courses(&self, platform: &Platform) -> Vec<String>
  {
    platform.enrollments.iter()
      .filter(|&e| {e.student.name == self.name})
      .map(|e| {e.course.name.clone()})
      .collect()
  }
}

struct Course
{
  name: String
}

impl Course
{
  fn new(name: String) -> Course
  {
    Course {name}
  }
}

struct Enrollment<'a>
{
  student: &'a Student,
  course: &'a Course
}
impl<'a> Enrollment<'a>
{
  fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a>
  {
    Enrollment {student, course}
  }
}

struct Platform<'a>
{
  enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a>
{
  fn new() -> Platform<'a>
  {
    Platform {enrollments: Vec::new()}
  }

  fn enroll(&mut self, student: &'a Student, course: &'a Course)
  {
    let enrollment = Enrollment::new(student, course);

    self.enrollments.push(enrollment);
  }
}

pub fn main()
{
  let John = Student::new("John".into());
  let rust_course = Course::new("Rust course".into());

  let mut p = Platform::new();
  p.enroll(&John, &rust_course);

  for course in John.get_registered_courses(&p)
  {
    println!("{} has registered {}", John.name, course);
  }
}
