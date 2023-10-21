use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage announcements in Google Classroom
    Announcement,

    /// View announcements in Google Classroom
    AnnouncementReadonly,

    /// See, edit, create, and permanently delete your Google Classroom classes
    Course,

    /// View your Google Classroom classes
    CourseReadonly,

    /// See, create and edit coursework items including assignments, questions, and grades
    CourseworkMe,

    /// View your course work and grades in Google Classroom
    CourseworkMeReadonly,

    /// Manage course work and grades for students in the Google Classroom classes you teach and view the course work and grades for classes you administer
    CourseworkStudent,

    /// View course work and grades for students in the Google Classroom classes you teach or administer
    CourseworkStudentReadonly,

    /// See, edit, and create classwork materials in Google Classroom
    Courseworkmaterial,

    /// See all classwork materials for your Google Classroom classes
    CourseworkmaterialReadonly,

    /// View your Google Classroom guardians
    GuardianlinkMeReadonly,

    /// View and manage guardians for students in your Google Classroom classes
    GuardianlinkStudent,

    /// View guardians for students in your Google Classroom classes
    GuardianlinkStudentReadonly,

    /// View the email addresses of people in your classes
    ProfileEmail,

    /// View the profile photos of people in your classes
    ProfilePhoto,

    /// Receive notifications about your Google Classroom data
    PushNotification,

    /// Manage your Google Classroom class rosters
    Roster,

    /// View your Google Classroom class rosters
    RosterReadonly,

    /// View your course work and grades in Google Classroom
    StudentSubmissionMeReadonly,

    /// View course work and grades for students in the Google Classroom classes you teach or administer
    StudentSubmissionStudentReadonly,

    /// See, create, and edit topics in Google Classroom
    Topic,

    /// View topics in Google Classroom
    TopicReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Announcement => "https://www.googleapis.com/auth/classroom.announcements",
            Scope::AnnouncementReadonly => "https://www.googleapis.com/auth/classroom.announcements.readonly",
            Scope::Course => "https://www.googleapis.com/auth/classroom.courses",
            Scope::CourseReadonly => "https://www.googleapis.com/auth/classroom.courses.readonly",
            Scope::CourseworkMe => "https://www.googleapis.com/auth/classroom.coursework.me",
            Scope::CourseworkMeReadonly => "https://www.googleapis.com/auth/classroom.coursework.me.readonly",
            Scope::CourseworkStudent => "https://www.googleapis.com/auth/classroom.coursework.students",
            Scope::CourseworkStudentReadonly => "https://www.googleapis.com/auth/classroom.coursework.students.readonly",
            Scope::Courseworkmaterial => "https://www.googleapis.com/auth/classroom.courseworkmaterials",
            Scope::CourseworkmaterialReadonly => "https://www.googleapis.com/auth/classroom.courseworkmaterials.readonly",
            Scope::GuardianlinkMeReadonly => "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly",
            Scope::GuardianlinkStudent => "https://www.googleapis.com/auth/classroom.guardianlinks.students",
            Scope::GuardianlinkStudentReadonly => "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly",
            Scope::ProfileEmail => "https://www.googleapis.com/auth/classroom.profile.emails",
            Scope::ProfilePhoto => "https://www.googleapis.com/auth/classroom.profile.photos",
            Scope::PushNotification => "https://www.googleapis.com/auth/classroom.push-notifications",
            Scope::Roster => "https://www.googleapis.com/auth/classroom.rosters",
            Scope::RosterReadonly => "https://www.googleapis.com/auth/classroom.rosters.readonly",
            Scope::StudentSubmissionMeReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly",
            Scope::StudentSubmissionStudentReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly",
            Scope::Topic => "https://www.googleapis.com/auth/classroom.topics",
            Scope::TopicReadonly => "https://www.googleapis.com/auth/classroom.topics.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AnnouncementReadonly
    }
}

