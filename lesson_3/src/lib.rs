use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    grade: u8,
}

#[derive(Debug, Clone)]
struct Course {
    id: u32,
    name: String,
    teacher: String,
}

// 社团
#[derive(Debug, Clone)]
struct StudentOrg {
    id: u32,
    name: String,
}

// 学生和社团是多对多关系
type StudentToOrg = HashMap<u32, Vec<u32>>;

// 学生和课程是多对多关系
type StudentToCourses = HashMap<u32, Vec<u32>>;

// 学生管理
#[derive(Debug, Clone)]
struct StudentManager {
    students: Vec<Student>,
    orgs: Vec<StudentOrg>,
    courses: Vec<Course>,
    student_to_orgs: StudentToOrg,
    student_to_courses: StudentToCourses,
}

impl StudentManager {
    fn new() -> Self {
        Self {
            students: vec![],
            orgs: vec![],
            courses: vec![],
            student_to_orgs: Default::default(),
            student_to_courses: Default::default(),
        }
    }

    // 创建学生，返回学生id
    fn create_student(&mut self, name: String, grade: u8) -> u32 {
        let id = self.students.len() as u32 + 1;
        self.students.push(Student { id, name, grade });
        id
    }

    // 根据id查询学生
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }

    // 更新学生年级
    fn update_student_grade(&mut self, id: u32, new_grade: u8) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.grade = new_grade;
        }
    }

    // 删除学生
    fn delete_student(&mut self, id: u32) {
        if let Some(idx) = self.students.iter().position(|s| s.id == id) {
            self.students.remove(idx);
        }
    }

    // 把学生加入社团
    fn enroll_org(&mut self, student_id: u32, org_id: u32) {
        self.student_to_orgs.entry(student_id).or_default().push(org_id);
        let org = StudentOrg {
            id: org_id,
            name: self.get_org(1).unwrap().name.clone()
        };
        self.orgs.push(org);
    }

    // 把学生加入课程
    fn enroll_course(&mut self, student_id: u32, course_id: u32) {
        self.student_to_courses.entry(student_id).or_default().push(course_id);
    }

    fn set_org(&mut self, org: StudentOrg) {
        let org = StudentOrg {
            id: org.id,
            name: org.name,
        };
        self.orgs.push(org);
    }

    // 获取社团信息
    fn get_org(&self, id: u32) -> Option<&StudentOrg> {
        self.orgs.iter().find(|c| c.id == id)
    }

    // 查询学生的社团
    // fn get_student_orgs(&self, student_id: u32) -> Vec<&StudentOrg> {
    fn get_student_orgs(&self, student_id: u32) -> Vec<&StudentOrg> {
        let org_ids = self.student_to_orgs.get(&student_id);
        let mut orgs = Vec::new();

        for id in org_ids.unwrap() {
            if let Some(org) = self.get_org(*id) {
                orgs.push(org);
            }
        }

        orgs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_test() {
        let mut system = StudentManager::new();
        let id = system.create_student("张三".to_string(), 2);
        let stu = system.get_student(id).unwrap();
        assert_eq!(stu.name, "张三");
        assert_eq!(stu.grade, 2);

        // 更新年级
        system.update_student_grade(id, 3);
        let stu = system.get_student(id).unwrap();
        assert_eq!(stu.grade, 3);

        // 创建社团
        let o1 = StudentOrg {
            id: 1,
            name: "集英社".to_string()
        };
        let o2 =  StudentOrg {
            id: 2,
            name: "映月社".to_string()
        };
        system.set_org(o1.clone());
        system.set_org(o2.clone());
        // println!("{:?}", system.orgs);

        // 查询社团信息
        let o1_info = system.get_org(o1.id);
        assert_eq!(1, o1_info.unwrap().id);
        assert_eq!("集英社".to_string(), o1_info.unwrap().name);

        // 张三加入了两个社团
        system.enroll_org(1, o1.id);
        system.enroll_org(1, o2.id);
        let orgs = system.get_student_orgs(1);
        assert_eq!(orgs.len(), 2);
        println!("{:?}", system);
    }
}