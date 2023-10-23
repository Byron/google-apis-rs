use super::*;
/// Announcement created by a teacher for students of the course
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [announcements create courses](CourseAnnouncementCreateCall) (request|response)
/// * [announcements get courses](CourseAnnouncementGetCall) (response)
/// * [announcements modify assignees courses](CourseAnnouncementModifyAssigneeCall) (response)
/// * [announcements patch courses](CourseAnnouncementPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Announcement {
    /// Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`.
    #[serde(rename="assigneeMode")]
    
    pub assignee_mode: Option<AnnouncementAssigneeModeEnum>,
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Timestamp when this announcement was created. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier for the user that created the announcement. Read-only.
    #[serde(rename="creatorUserId")]
    
    pub creator_user_id: Option<String>,
    /// Classroom-assigned identifier of this announcement, unique per course. Read-only.
    
    pub id: Option<String>,
    /// Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the announcement.
    #[serde(rename="individualStudentsOptions")]
    
    pub individual_students_options: Option<IndividualStudentsOptions>,
    /// Additional materials. Announcements must have no more than 20 material items.
    
    pub materials: Option<Vec<Material>>,
    /// Optional timestamp when this announcement is scheduled to be published.
    #[serde(rename="scheduledTime")]
    
    pub scheduled_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of this announcement. If unspecified, the default state is `DRAFT`.
    
    pub state: Option<AnnouncementStateEnum>,
    /// Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters.
    
    pub text: Option<String>,
    /// Timestamp of the most recent change to this announcement. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Announcement {}
impl client::ResponseResult for Announcement {}


/// Additional details for assignments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// Drive folder where attachments from student submissions are placed. This is only populated for course teachers and administrators.
    #[serde(rename="studentWorkFolder")]
    
    pub student_work_folder: Option<DriveFolder>,
}

impl client::Part for Assignment {}


/// Student work for an assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignmentSubmission {
    /// Attachments added by the student. Drive files that correspond to materials with a share mode of STUDENT_COPY may not exist yet if the student has not accessed the assignment in Classroom. Some attachment metadata is only populated if the requesting user has permission to access it. Identifier and alternate_link fields are always available, but others (for example, title) may not be.
    
    pub attachments: Option<Vec<Attachment>>,
}

impl client::Part for AssignmentSubmission {}


/// Attachment added to student assignment work. When creating attachments, setting the `form` field is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// Google Drive file attachment.
    #[serde(rename="driveFile")]
    
    pub drive_file: Option<DriveFile>,
    /// Google Forms attachment.
    
    pub form: Option<Form>,
    /// Link attachment.
    
    pub link: Option<Link>,
    /// Youtube video attachment.
    #[serde(rename="youTubeVideo")]
    
    pub you_tube_video: Option<YouTubeVideo>,
}

impl client::Part for Attachment {}


/// A reference to a Cloud Pub/Sub topic. To register for notifications, the owner of the topic must grant `classroom-notifications@system.gserviceaccount.com` the `projects.topics.publish` permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudPubsubTopic {
    /// The `name` field of a Cloud Pub/Sub [Topic](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic).
    #[serde(rename="topicName")]
    
    pub topic_name: Option<String>,
}

impl client::Part for CloudPubsubTopic {}


/// A Course in Classroom.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases create courses](CourseAliasCreateCall) (none)
/// * [aliases delete courses](CourseAliasDeleteCall) (none)
/// * [aliases list courses](CourseAliasListCall) (none)
/// * [announcements create courses](CourseAnnouncementCreateCall) (none)
/// * [announcements delete courses](CourseAnnouncementDeleteCall) (none)
/// * [announcements get courses](CourseAnnouncementGetCall) (none)
/// * [announcements list courses](CourseAnnouncementListCall) (none)
/// * [announcements modify assignees courses](CourseAnnouncementModifyAssigneeCall) (none)
/// * [announcements patch courses](CourseAnnouncementPatchCall) (none)
/// * [course work student submissions get courses](CourseCourseWorkStudentSubmissionGetCall) (none)
/// * [course work student submissions list courses](CourseCourseWorkStudentSubmissionListCall) (none)
/// * [course work student submissions modify attachments courses](CourseCourseWorkStudentSubmissionModifyAttachmentCall) (none)
/// * [course work student submissions patch courses](CourseCourseWorkStudentSubmissionPatchCall) (none)
/// * [course work student submissions reclaim courses](CourseCourseWorkStudentSubmissionReclaimCall) (none)
/// * [course work student submissions return courses](CourseCourseWorkStudentSubmissionReturnCall) (none)
/// * [course work student submissions turn in courses](CourseCourseWorkStudentSubmissionTurnInCall) (none)
/// * [course work create courses](CourseCourseWorkCreateCall) (none)
/// * [course work delete courses](CourseCourseWorkDeleteCall) (none)
/// * [course work get courses](CourseCourseWorkGetCall) (none)
/// * [course work list courses](CourseCourseWorkListCall) (none)
/// * [course work modify assignees courses](CourseCourseWorkModifyAssigneeCall) (none)
/// * [course work patch courses](CourseCourseWorkPatchCall) (none)
/// * [course work materials create courses](CourseCourseWorkMaterialCreateCall) (none)
/// * [course work materials delete courses](CourseCourseWorkMaterialDeleteCall) (none)
/// * [course work materials get courses](CourseCourseWorkMaterialGetCall) (none)
/// * [course work materials list courses](CourseCourseWorkMaterialListCall) (none)
/// * [course work materials patch courses](CourseCourseWorkMaterialPatchCall) (none)
/// * [students create courses](CourseStudentCreateCall) (none)
/// * [students delete courses](CourseStudentDeleteCall) (none)
/// * [students get courses](CourseStudentGetCall) (none)
/// * [students list courses](CourseStudentListCall) (none)
/// * [teachers create courses](CourseTeacherCreateCall) (none)
/// * [teachers delete courses](CourseTeacherDeleteCall) (none)
/// * [teachers get courses](CourseTeacherGetCall) (none)
/// * [teachers list courses](CourseTeacherListCall) (none)
/// * [topics create courses](CourseTopicCreateCall) (none)
/// * [topics delete courses](CourseTopicDeleteCall) (none)
/// * [topics get courses](CourseTopicGetCall) (none)
/// * [topics list courses](CourseTopicListCall) (none)
/// * [topics patch courses](CourseTopicPatchCall) (none)
/// * [create courses](CourseCreateCall) (request|response)
/// * [delete courses](CourseDeleteCall) (none)
/// * [get courses](CourseGetCall) (response)
/// * [list courses](CourseListCall) (none)
/// * [patch courses](CoursePatchCall) (request|response)
/// * [update courses](CourseUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    /// Absolute link to this course in the Classroom web UI. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. Read-only.
    #[serde(rename="calendarId")]
    
    pub calendar_id: Option<String>,
    /// The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only.
    #[serde(rename="courseGroupEmail")]
    
    pub course_group_email: Option<String>,
    /// Sets of materials that appear on the "about" page of this course. Read-only.
    #[serde(rename="courseMaterialSets")]
    
    pub course_material_sets: Option<Vec<CourseMaterialSet>>,
    /// State of the course. If unspecified, the default state is `PROVISIONED`.
    #[serde(rename="courseState")]
    
    pub course_state: Option<CourseCourseStateEnum>,
    /// Creation time of the course. Specifying this field in a course update mask results in an error. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters.
    
    pub description: Option<String>,
    /// Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters.
    #[serde(rename="descriptionHeading")]
    
    pub description_heading: Option<String>,
    /// Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only.
    #[serde(rename="enrollmentCode")]
    
    pub enrollment_code: Option<String>,
    /// The gradebook settings that specify how a student's overall grade for the course will be calculated and who it will be displayed to. Read-only
    #[serde(rename="gradebookSettings")]
    
    pub gradebook_settings: Option<GradebookSettings>,
    /// Whether or not guardian notifications are enabled for this course. Read-only.
    #[serde(rename="guardiansEnabled")]
    
    pub guardians_enabled: Option<bool>,
    /// Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The `id` is still assigned by Classroom and cannot be updated after the course is created. Specifying this field in a course update mask results in an error.
    
    pub id: Option<String>,
    /// Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string.
    
    pub name: Option<String>,
    /// The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user This must be set in a create request. Admins can also specify this field in a patch course request to transfer ownership. In other contexts, it is read-only.
    #[serde(rename="ownerId")]
    
    pub owner_id: Option<String>,
    /// Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters.
    
    pub room: Option<String>,
    /// Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters.
    
    pub section: Option<String>,
    /// Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only.
    #[serde(rename="teacherFolder")]
    
    pub teacher_folder: Option<DriveFolder>,
    /// The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only.
    #[serde(rename="teacherGroupEmail")]
    
    pub teacher_group_email: Option<String>,
    /// Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Course {}
impl client::Resource for Course {}
impl client::ResponseResult for Course {}


/// Alternative identifier for a course. An alias uniquely identifies a course. It must be unique within one of the following scopes: * domain: A domain-scoped alias is visible to all users within the alias creator’s domain and can be created only by a domain admin. A domain-scoped alias is often used when a course has an identifier external to Classroom. * project: A project-scoped alias is visible to any request from an application using the Developer Console project ID that created the alias and can be created by any project. A project-scoped alias is often used when an application has alternative identifiers. A random value can also be used to avoid duplicate courses in the event of transmission failures, as retrying a request will return `ALREADY_EXISTS` if a previous one has succeeded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases create courses](CourseAliasCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseAlias {
    /// Alias string. The format of the string indicates the desired alias scoping. * `d:` indicates a domain-scoped alias. Example: `d:math_101` * `p:` indicates a project-scoped alias. Example: `p:abc123` This field has a maximum length of 256 characters.
    
    pub alias: Option<String>,
}

impl client::RequestValue for CourseAlias {}
impl client::ResponseResult for CourseAlias {}


/// A material attached to a course as part of a material set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseMaterial {
    /// Google Drive file attachment.
    #[serde(rename="driveFile")]
    
    pub drive_file: Option<DriveFile>,
    /// Google Forms attachment.
    
    pub form: Option<Form>,
    /// Link atatchment.
    
    pub link: Option<Link>,
    /// Youtube video attachment.
    #[serde(rename="youTubeVideo")]
    
    pub you_tube_video: Option<YouTubeVideo>,
}

impl client::Part for CourseMaterial {}


/// A set of materials that appears on the "About" page of the course. These materials might include a syllabus, schedule, or other background information relating to the course as a whole.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseMaterialSet {
    /// Materials attached to this set.
    
    pub materials: Option<Vec<CourseMaterial>>,
    /// Title for this set.
    
    pub title: Option<String>,
}

impl client::Part for CourseMaterialSet {}


/// Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseRosterChangesInfo {
    /// The `course_id` of the course to subscribe to roster changes for.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
}

impl client::Part for CourseRosterChangesInfo {}


/// Course work created by a teacher for students of the course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work create courses](CourseCourseWorkCreateCall) (request|response)
/// * [course work get courses](CourseCourseWorkGetCall) (response)
/// * [course work modify assignees courses](CourseCourseWorkModifyAssigneeCall) (response)
/// * [course work patch courses](CourseCourseWorkPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseWork {
    /// Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`.
    #[serde(rename="assigneeMode")]
    
    pub assignee_mode: Option<CourseWorkAssigneeModeEnum>,
    /// Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only.
    
    pub assignment: Option<Assignment>,
    /// Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only.
    #[serde(rename="associatedWithDeveloper")]
    
    pub associated_with_developer: Option<bool>,
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Timestamp when this course work was created. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier for the user that created the coursework. Read-only.
    #[serde(rename="creatorUserId")]
    
    pub creator_user_id: Option<String>,
    /// Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters.
    
    pub description: Option<String>,
    /// Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified.
    #[serde(rename="dueDate")]
    
    pub due_date: Option<Date>,
    /// Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified.
    #[serde(rename="dueTime")]
    
    pub due_time: Option<TimeOfDay>,
    /// The category that this coursework's grade contributes to. Present only when a category has been chosen for the coursework. May be used in calculating the overall grade. Read-only.
    #[serde(rename="gradeCategory")]
    
    pub grade_category: Option<GradeCategory>,
    /// Classroom-assigned identifier of this course work, unique per course. Read-only.
    
    pub id: Option<String>,
    /// Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field are assigned the coursework.
    #[serde(rename="individualStudentsOptions")]
    
    pub individual_students_options: Option<IndividualStudentsOptions>,
    /// Additional materials. CourseWork must have no more than 20 material items.
    
    pub materials: Option<Vec<Material>>,
    /// Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value.
    #[serde(rename="maxPoints")]
    
    pub max_points: Option<f64>,
    /// Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. For write operations, this field must be specified when creating course work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be set otherwise.
    #[serde(rename="multipleChoiceQuestion")]
    
    pub multiple_choice_question: Option<MultipleChoiceQuestion>,
    /// Optional timestamp when this course work is scheduled to be published.
    #[serde(rename="scheduledTime")]
    
    pub scheduled_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of this course work. If unspecified, the default state is `DRAFT`.
    
    pub state: Option<CourseWorkStateEnum>,
    /// Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`.
    #[serde(rename="submissionModificationMode")]
    
    pub submission_modification_mode: Option<CourseWorkSubmissionModificationModeEnum>,
    /// Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters.
    
    pub title: Option<String>,
    /// Identifier for the topic that this coursework is associated with. Must match an existing topic in the course.
    #[serde(rename="topicId")]
    
    pub topic_id: Option<String>,
    /// Timestamp of the most recent change to this course work. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Type of this course work. The type is set when the course work is created and cannot be changed.
    #[serde(rename="workType")]
    
    pub work_type: Option<CourseWorkWorkTypeEnum>,
}

impl client::RequestValue for CourseWork {}
impl client::ResponseResult for CourseWork {}


/// Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseWorkChangesInfo {
    /// The `course_id` of the course to subscribe to work changes for.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
}

impl client::Part for CourseWorkChangesInfo {}


/// Course work material created by a teacher for students of the course
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work materials create courses](CourseCourseWorkMaterialCreateCall) (request|response)
/// * [course work materials get courses](CourseCourseWorkMaterialGetCall) (response)
/// * [course work materials patch courses](CourseCourseWorkMaterialPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseWorkMaterial {
    /// Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`.
    #[serde(rename="assigneeMode")]
    
    pub assignee_mode: Option<CourseWorkMaterialAssigneeModeEnum>,
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Timestamp when this course work material was created. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier for the user that created the course work material. Read-only.
    #[serde(rename="creatorUserId")]
    
    pub creator_user_id: Option<String>,
    /// Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters.
    
    pub description: Option<String>,
    /// Classroom-assigned identifier of this course work material, unique per course. Read-only.
    
    pub id: Option<String>,
    /// Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the course work material.
    #[serde(rename="individualStudentsOptions")]
    
    pub individual_students_options: Option<IndividualStudentsOptions>,
    /// Additional materials. A course work material must have no more than 20 material items.
    
    pub materials: Option<Vec<Material>>,
    /// Optional timestamp when this course work material is scheduled to be published.
    #[serde(rename="scheduledTime")]
    
    pub scheduled_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Status of this course work material. If unspecified, the default state is `DRAFT`.
    
    pub state: Option<CourseWorkMaterialStateEnum>,
    /// Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters.
    
    pub title: Option<String>,
    /// Identifier for the topic that this course work material is associated with. Must match an existing topic in the course.
    #[serde(rename="topicId")]
    
    pub topic_id: Option<String>,
    /// Timestamp of the most recent change to this course work material. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CourseWorkMaterial {}
impl client::ResponseResult for CourseWorkMaterial {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// Representation of a Google Drive file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFile {
    /// URL that can be used to access the Drive item. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Drive API resource ID.
    
    pub id: Option<String>,
    /// URL of a thumbnail image of the Drive item. Read-only.
    #[serde(rename="thumbnailUrl")]
    
    pub thumbnail_url: Option<String>,
    /// Title of the Drive item. Read-only.
    
    pub title: Option<String>,
}

impl client::Part for DriveFile {}


/// Representation of a Google Drive folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFolder {
    /// URL that can be used to access the Drive folder. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Drive API resource ID.
    
    pub id: Option<String>,
    /// Title of the Drive folder. Read-only.
    
    pub title: Option<String>,
}

impl client::Part for DriveFolder {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases delete courses](CourseAliasDeleteCall) (response)
/// * [announcements delete courses](CourseAnnouncementDeleteCall) (response)
/// * [course work student submissions reclaim courses](CourseCourseWorkStudentSubmissionReclaimCall) (response)
/// * [course work student submissions return courses](CourseCourseWorkStudentSubmissionReturnCall) (response)
/// * [course work student submissions turn in courses](CourseCourseWorkStudentSubmissionTurnInCall) (response)
/// * [course work delete courses](CourseCourseWorkDeleteCall) (response)
/// * [course work materials delete courses](CourseCourseWorkMaterialDeleteCall) (response)
/// * [students delete courses](CourseStudentDeleteCall) (response)
/// * [teachers delete courses](CourseTeacherDeleteCall) (response)
/// * [topics delete courses](CourseTopicDeleteCall) (response)
/// * [delete courses](CourseDeleteCall) (response)
/// * [accept invitations](InvitationAcceptCall) (response)
/// * [delete invitations](InvitationDeleteCall) (response)
/// * [delete registrations](RegistrationDeleteCall) (response)
/// * [guardians delete user profiles](UserProfileGuardianDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A class of notifications that an application can register to receive. For example: "all roster changes for a domain".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feed {
    /// Information about a `Feed` with a `feed_type` of `COURSE_ROSTER_CHANGES`. This field must be specified if `feed_type` is `COURSE_ROSTER_CHANGES`.
    #[serde(rename="courseRosterChangesInfo")]
    
    pub course_roster_changes_info: Option<CourseRosterChangesInfo>,
    /// Information about a `Feed` with a `feed_type` of `COURSE_WORK_CHANGES`. This field must be specified if `feed_type` is `COURSE_WORK_CHANGES`.
    #[serde(rename="courseWorkChangesInfo")]
    
    pub course_work_changes_info: Option<CourseWorkChangesInfo>,
    /// The type of feed.
    #[serde(rename="feedType")]
    
    pub feed_type: Option<FeedFeedTypeEnum>,
}

impl client::Part for Feed {}


/// Google Forms item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    /// URL of the form.
    #[serde(rename="formUrl")]
    
    pub form_url: Option<String>,
    /// URL of the form responses document. Only set if respsonses have been recorded and only when the requesting user is an editor of the form. Read-only.
    #[serde(rename="responseUrl")]
    
    pub response_url: Option<String>,
    /// URL of a thumbnail image of the Form. Read-only.
    #[serde(rename="thumbnailUrl")]
    
    pub thumbnail_url: Option<String>,
    /// Title of the Form. Read-only.
    
    pub title: Option<String>,
}

impl client::Part for Form {}


/// Global user permission description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlobalPermission {
    /// Permission value.
    
    pub permission: Option<GlobalPermissionPermissionEnum>,
}

impl client::Part for GlobalPermission {}


/// Details for a grade category in a course. Coursework may have zero or one grade category, and the category may be used in computing the overall grade. See the [help center article](https://support.google.com/edu/classroom/answer/9184995) for details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GradeCategory {
    /// Default value of denominator. Only applicable when grade calculation type is TOTAL_POINTS.
    #[serde(rename="defaultGradeDenominator")]
    
    pub default_grade_denominator: Option<i32>,
    /// ID of the grade category.
    
    pub id: Option<String>,
    /// Name of the grade category.
    
    pub name: Option<String>,
    /// The weight of the category average as part of overall average. A weight of 12.34% is represented as 123400 (100% is 1,000,000). The last two digits should always be zero since we use two decimal precision. Only applicable when grade calculation type is WEIGHTED_CATEGORIES.
    
    pub weight: Option<i32>,
}

impl client::Part for GradeCategory {}


/// The history of each grade on this submission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GradeHistory {
    /// The teacher who made the grade change.
    #[serde(rename="actorUserId")]
    
    pub actor_user_id: Option<String>,
    /// The type of grade change at this time in the submission grade history.
    #[serde(rename="gradeChangeType")]
    
    pub grade_change_type: Option<GradeHistoryGradeChangeTypeEnum>,
    /// When the grade of the submission was changed.
    #[serde(rename="gradeTimestamp")]
    
    pub grade_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The denominator of the grade at this time in the submission grade history.
    #[serde(rename="maxPoints")]
    
    pub max_points: Option<f64>,
    /// The numerator of the grade at this time in the submission grade history.
    #[serde(rename="pointsEarned")]
    
    pub points_earned: Option<f64>,
}

impl client::Part for GradeHistory {}


/// The gradebook settings for a course. See the [help center article](https://support.google.com/edu/classroom/answer/9184995) for details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GradebookSettings {
    /// Indicates how the overall grade is calculated.
    #[serde(rename="calculationType")]
    
    pub calculation_type: Option<GradebookSettingCalculationTypeEnum>,
    /// Indicates who can see the overall grade..
    #[serde(rename="displaySetting")]
    
    pub display_setting: Option<GradebookSettingDisplaySettingEnum>,
    /// Grade categories that are available for coursework in the course.
    #[serde(rename="gradeCategories")]
    
    pub grade_categories: Option<Vec<GradeCategory>>,
}

impl client::Part for GradebookSettings {}


/// Association between a student and a guardian of that student. The guardian may receive information about the student’s course work.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardians get user profiles](UserProfileGuardianGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Guardian {
    /// Identifier for the guardian.
    #[serde(rename="guardianId")]
    
    pub guardian_id: Option<String>,
    /// User profile for the guardian.
    #[serde(rename="guardianProfile")]
    
    pub guardian_profile: Option<UserProfile>,
    /// The email address to which the initial guardian invitation was sent. This field is only visible to domain administrators.
    #[serde(rename="invitedEmailAddress")]
    
    pub invited_email_address: Option<String>,
    /// Identifier for the student to whom the guardian relationship applies.
    #[serde(rename="studentId")]
    
    pub student_id: Option<String>,
}

impl client::ResponseResult for Guardian {}


/// An invitation to become the guardian of a specified user, sent to a specified email address.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations create user profiles](UserProfileGuardianInvitationCreateCall) (request|response)
/// * [guardian invitations get user profiles](UserProfileGuardianInvitationGetCall) (response)
/// * [guardian invitations patch user profiles](UserProfileGuardianInvitationPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuardianInvitation {
    /// The time that this invitation was created. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unique identifier for this invitation. Read-only.
    #[serde(rename="invitationId")]
    
    pub invitation_id: Option<String>,
    /// Email address that the invitation was sent to. This field is only visible to domain administrators.
    #[serde(rename="invitedEmailAddress")]
    
    pub invited_email_address: Option<String>,
    /// The state that this invitation is in.
    
    pub state: Option<GuardianInvitationStateEnum>,
    /// ID of the student (in standard format)
    #[serde(rename="studentId")]
    
    pub student_id: Option<String>,
}

impl client::RequestValue for GuardianInvitation {}
impl client::ResponseResult for GuardianInvitation {}


/// Assignee details about a coursework/announcement. This field is set if and only if `assigneeMode` is `INDIVIDUAL_STUDENTS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndividualStudentsOptions {
    /// Identifiers for the students that have access to the coursework/announcement.
    #[serde(rename="studentIds")]
    
    pub student_ids: Option<Vec<String>>,
}

impl client::Part for IndividualStudentsOptions {}


/// An invitation to join a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accept invitations](InvitationAcceptCall) (none)
/// * [create invitations](InvitationCreateCall) (request|response)
/// * [delete invitations](InvitationDeleteCall) (none)
/// * [get invitations](InvitationGetCall) (response)
/// * [list invitations](InvitationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Invitation {
    /// Identifier of the course to invite the user to.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Identifier assigned by Classroom. Read-only.
    
    pub id: Option<String>,
    /// Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`.
    
    pub role: Option<InvitationRoleEnum>,
    /// Identifier of the invited user. When specified as a parameter of a request, this identifier can be set to one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for Invitation {}
impl client::Resource for Invitation {}
impl client::ResponseResult for Invitation {}


/// URL item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// URL of a thumbnail image of the target URL. Read-only.
    #[serde(rename="thumbnailUrl")]
    
    pub thumbnail_url: Option<String>,
    /// Title of the target of the URL. Read-only.
    
    pub title: Option<String>,
    /// URL to link to. This must be a valid UTF-8 string containing between 1 and 2024 characters.
    
    pub url: Option<String>,
}

impl client::Part for Link {}


/// Response when listing course work.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [announcements list courses](CourseAnnouncementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAnnouncementsResponse {
    /// Announcement items that match the request.
    
    pub announcements: Option<Vec<Announcement>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAnnouncementsResponse {}


/// Response when listing course aliases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases list courses](CourseAliasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCourseAliasesResponse {
    /// The course aliases.
    
    pub aliases: Option<Vec<CourseAlias>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCourseAliasesResponse {}


/// Response when listing course work material.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work materials list courses](CourseCourseWorkMaterialListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCourseWorkMaterialResponse {
    /// Course work material items that match the request.
    #[serde(rename="courseWorkMaterial")]
    
    pub course_work_material: Option<Vec<CourseWorkMaterial>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCourseWorkMaterialResponse {}


/// Response when listing course work.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work list courses](CourseCourseWorkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCourseWorkResponse {
    /// Course work items that match the request.
    #[serde(rename="courseWork")]
    
    pub course_work: Option<Vec<CourseWork>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCourseWorkResponse {}


/// Response when listing courses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list courses](CourseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCoursesResponse {
    /// Courses that match the list request.
    
    pub courses: Option<Vec<Course>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCoursesResponse {}


/// Response when listing guardian invitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations list user profiles](UserProfileGuardianInvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGuardianInvitationsResponse {
    /// Guardian invitations that matched the list request.
    #[serde(rename="guardianInvitations")]
    
    pub guardian_invitations: Option<Vec<GuardianInvitation>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGuardianInvitationsResponse {}


/// Response when listing guardians.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardians list user profiles](UserProfileGuardianListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGuardiansResponse {
    /// Guardians on this page of results that met the criteria specified in the request.
    
    pub guardians: Option<Vec<Guardian>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGuardiansResponse {}


/// Response when listing invitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list invitations](InvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInvitationsResponse {
    /// Invitations that match the list request.
    
    pub invitations: Option<Vec<Invitation>>,
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInvitationsResponse {}


/// Response when listing student submissions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions list courses](CourseCourseWorkStudentSubmissionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStudentSubmissionsResponse {
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Student work that matches the request.
    #[serde(rename="studentSubmissions")]
    
    pub student_submissions: Option<Vec<StudentSubmission>>,
}

impl client::ResponseResult for ListStudentSubmissionsResponse {}


/// Response when listing students.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [students list courses](CourseStudentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStudentsResponse {
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Students who match the list request.
    
    pub students: Option<Vec<Student>>,
}

impl client::ResponseResult for ListStudentsResponse {}


/// Response when listing teachers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [teachers list courses](CourseTeacherListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTeachersResponse {
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Teachers who match the list request.
    
    pub teachers: Option<Vec<Teacher>>,
}

impl client::ResponseResult for ListTeachersResponse {}


/// Response when listing topics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [topics list courses](CourseTopicListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTopicResponse {
    /// Token identifying the next page of results to return. If empty, no further results are available.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Topic items that match the request.
    
    pub topic: Option<Vec<Topic>>,
}

impl client::ResponseResult for ListTopicResponse {}


/// Material attached to course work. When creating attachments, setting the `form` field is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    /// Google Drive file material.
    #[serde(rename="driveFile")]
    
    pub drive_file: Option<SharedDriveFile>,
    /// Google Forms material.
    
    pub form: Option<Form>,
    /// Link material. On creation, this is upgraded to a more appropriate type if possible, and this is reflected in the response.
    
    pub link: Option<Link>,
    /// YouTube video material.
    #[serde(rename="youtubeVideo")]
    
    pub youtube_video: Option<YouTubeVideo>,
}

impl client::Part for Material {}


/// Request to modify assignee mode and options of an announcement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [announcements modify assignees courses](CourseAnnouncementModifyAssigneeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyAnnouncementAssigneesRequest {
    /// Mode of the announcement describing whether it is accessible by all students or specified individual students.
    #[serde(rename="assigneeMode")]
    
    pub assignee_mode: Option<ModifyAnnouncementAssigneesRequestAssigneeModeEnum>,
    /// Set which students can view or cannot view the announcement. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`.
    #[serde(rename="modifyIndividualStudentsOptions")]
    
    pub modify_individual_students_options: Option<ModifyIndividualStudentsOptions>,
}

impl client::RequestValue for ModifyAnnouncementAssigneesRequest {}


/// Request to modify the attachments of a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions modify attachments courses](CourseCourseWorkStudentSubmissionModifyAttachmentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyAttachmentsRequest {
    /// Attachments to add. A student submission may not have more than 20 attachments. Form attachments are not supported.
    #[serde(rename="addAttachments")]
    
    pub add_attachments: Option<Vec<Attachment>>,
}

impl client::RequestValue for ModifyAttachmentsRequest {}


/// Request to modify assignee mode and options of a coursework.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work modify assignees courses](CourseCourseWorkModifyAssigneeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyCourseWorkAssigneesRequest {
    /// Mode of the coursework describing whether it will be assigned to all students or specified individual students.
    #[serde(rename="assigneeMode")]
    
    pub assignee_mode: Option<ModifyCourseWorkAssigneesRequestAssigneeModeEnum>,
    /// Set which students are assigned or not assigned to the coursework. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`.
    #[serde(rename="modifyIndividualStudentsOptions")]
    
    pub modify_individual_students_options: Option<ModifyIndividualStudentsOptions>,
}

impl client::RequestValue for ModifyCourseWorkAssigneesRequest {}


/// Contains fields to add or remove students from a course work or announcement where the `assigneeMode` is set to `INDIVIDUAL_STUDENTS`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyIndividualStudentsOptions {
    /// IDs of students to be added as having access to this coursework/announcement.
    #[serde(rename="addStudentIds")]
    
    pub add_student_ids: Option<Vec<String>>,
    /// IDs of students to be removed from having access to this coursework/announcement.
    #[serde(rename="removeStudentIds")]
    
    pub remove_student_ids: Option<Vec<String>>,
}

impl client::Part for ModifyIndividualStudentsOptions {}


/// Additional details for multiple-choice questions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    /// Possible choices.
    
    pub choices: Option<Vec<String>>,
}

impl client::Part for MultipleChoiceQuestion {}


/// Student work for a multiple-choice question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultipleChoiceSubmission {
    /// Student's select choice.
    
    pub answer: Option<String>,
}

impl client::Part for MultipleChoiceSubmission {}


/// Details of the user's name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// The user's last name. Read-only.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The user's full name formed by concatenating the first and last name values. Read-only.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// The user's first name. Read-only.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
}

impl client::Part for Name {}


/// Request to reclaim a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions reclaim courses](CourseCourseWorkStudentSubmissionReclaimCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReclaimStudentSubmissionRequest { _never_set: Option<bool> }

impl client::RequestValue for ReclaimStudentSubmissionRequest {}


/// An instruction to Classroom to send notifications from the `feed` to the provided destination.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create registrations](RegistrationCreateCall) (request|response)
/// * [delete registrations](RegistrationDeleteCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Registration {
    /// The Cloud Pub/Sub topic that notifications are to be sent to.
    #[serde(rename="cloudPubsubTopic")]
    
    pub cloud_pubsub_topic: Option<CloudPubsubTopic>,
    /// The time until which the `Registration` is effective. This is a read-only field assigned by the server.
    #[serde(rename="expiryTime")]
    
    pub expiry_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Specification for the class of notifications that Classroom should deliver to the destination.
    
    pub feed: Option<Feed>,
    /// A server-generated unique identifier for this `Registration`. Read-only.
    #[serde(rename="registrationId")]
    
    pub registration_id: Option<String>,
}

impl client::RequestValue for Registration {}
impl client::Resource for Registration {}
impl client::ResponseResult for Registration {}


/// Request to return a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions return courses](CourseCourseWorkStudentSubmissionReturnCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReturnStudentSubmissionRequest { _never_set: Option<bool> }

impl client::RequestValue for ReturnStudentSubmissionRequest {}


/// Drive file that is used as material for course work.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedDriveFile {
    /// Drive file details.
    #[serde(rename="driveFile")]
    
    pub drive_file: Option<DriveFile>,
    /// Mechanism by which students access the Drive item.
    #[serde(rename="shareMode")]
    
    pub share_mode: Option<SharedDriveFileShareModeEnum>,
}

impl client::Part for SharedDriveFile {}


/// Student work for a short answer question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShortAnswerSubmission {
    /// Student response to a short-answer question.
    
    pub answer: Option<String>,
}

impl client::Part for ShortAnswerSubmission {}


/// The history of each state this submission has been in.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StateHistory {
    /// The teacher or student who made the change.
    #[serde(rename="actorUserId")]
    
    pub actor_user_id: Option<String>,
    /// The workflow pipeline stage.
    
    pub state: Option<StateHistoryStateEnum>,
    /// When the submission entered this state.
    #[serde(rename="stateTimestamp")]
    
    pub state_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for StateHistory {}


/// Student in a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [students create courses](CourseStudentCreateCall) (request|response)
/// * [students get courses](CourseStudentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Global user information for the student. Read-only.
    
    pub profile: Option<UserProfile>,
    /// Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only.
    #[serde(rename="studentWorkFolder")]
    
    pub student_work_folder: Option<DriveFolder>,
    /// Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for Student {}
impl client::ResponseResult for Student {}


/// Student submission for course work. StudentSubmission items are generated when a CourseWork item is created. StudentSubmissions that have never been accessed (i.e. with `state` = NEW) may not have a creation time or update time.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions get courses](CourseCourseWorkStudentSubmissionGetCall) (response)
/// * [course work student submissions modify attachments courses](CourseCourseWorkStudentSubmissionModifyAttachmentCall) (response)
/// * [course work student submissions patch courses](CourseCourseWorkStudentSubmissionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StudentSubmission {
    /// Absolute link to the submission in the Classroom web UI. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// Optional grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This may be modified only by course teachers.
    #[serde(rename="assignedGrade")]
    
    pub assigned_grade: Option<f64>,
    /// Submission content when course_work_type is ASSIGNMENT. Students can modify this content using ModifyAttachments.
    #[serde(rename="assignmentSubmission")]
    
    pub assignment_submission: Option<AssignmentSubmission>,
    /// Whether this student submission is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only.
    #[serde(rename="associatedWithDeveloper")]
    
    pub associated_with_developer: Option<bool>,
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Identifier for the course work this corresponds to. Read-only.
    #[serde(rename="courseWorkId")]
    
    pub course_work_id: Option<String>,
    /// Type of course work this submission is for. Read-only.
    #[serde(rename="courseWorkType")]
    
    pub course_work_type: Option<StudentSubmissionCourseWorkTypeEnum>,
    /// Creation time of this submission. This may be unset if the student has not accessed this item. Read-only.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional pending grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. This is only visible to and modifiable by course teachers.
    #[serde(rename="draftGrade")]
    
    pub draft_grade: Option<f64>,
    /// Classroom-assigned Identifier for the student submission. This is unique among submissions for the relevant course work. Read-only.
    
    pub id: Option<String>,
    /// Whether this submission is late. Read-only.
    
    pub late: Option<bool>,
    /// Submission content when course_work_type is MULTIPLE_CHOICE_QUESTION.
    #[serde(rename="multipleChoiceSubmission")]
    
    pub multiple_choice_submission: Option<MultipleChoiceSubmission>,
    /// Submission content when course_work_type is SHORT_ANSWER_QUESTION.
    #[serde(rename="shortAnswerSubmission")]
    
    pub short_answer_submission: Option<ShortAnswerSubmission>,
    /// State of this submission. Read-only.
    
    pub state: Option<StudentSubmissionStateEnum>,
    /// The history of the submission (includes state and grade histories). Read-only.
    #[serde(rename="submissionHistory")]
    
    pub submission_history: Option<Vec<SubmissionHistory>>,
    /// Last update time of this submission. This may be unset if the student has not accessed this item. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifier for the student that owns this submission. Read-only.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for StudentSubmission {}
impl client::ResponseResult for StudentSubmission {}


/// The history of the submission. This currently includes state and grade histories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubmissionHistory {
    /// The grade history information of the submission, if present.
    #[serde(rename="gradeHistory")]
    
    pub grade_history: Option<GradeHistory>,
    /// The state history information of the submission, if present.
    #[serde(rename="stateHistory")]
    
    pub state_history: Option<StateHistory>,
}

impl client::Part for SubmissionHistory {}


/// Teacher of a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [teachers create courses](CourseTeacherCreateCall) (request|response)
/// * [teachers get courses](CourseTeacherGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Teacher {
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// Global user information for the teacher. Read-only.
    
    pub profile: Option<UserProfile>,
    /// Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for Teacher {}
impl client::ResponseResult for Teacher {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Topic created by a teacher for the course
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [topics create courses](CourseTopicCreateCall) (request|response)
/// * [topics get courses](CourseTopicGetCall) (response)
/// * [topics patch courses](CourseTopicPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    /// Identifier of the course. Read-only.
    #[serde(rename="courseId")]
    
    pub course_id: Option<String>,
    /// The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. The result must be a non-empty string. Topic names are case sensitive, and must be no longer than 100 characters.
    
    pub name: Option<String>,
    /// Unique identifier for the topic. Read-only.
    #[serde(rename="topicId")]
    
    pub topic_id: Option<String>,
    /// The time the topic was last updated by the system. Read-only.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Topic {}
impl client::ResponseResult for Topic {}


/// Request to turn in a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions turn in courses](CourseCourseWorkStudentSubmissionTurnInCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TurnInStudentSubmissionRequest { _never_set: Option<bool> }

impl client::RequestValue for TurnInStudentSubmissionRequest {}


/// Global information for a user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations create user profiles](UserProfileGuardianInvitationCreateCall) (none)
/// * [guardian invitations get user profiles](UserProfileGuardianInvitationGetCall) (none)
/// * [guardian invitations list user profiles](UserProfileGuardianInvitationListCall) (none)
/// * [guardian invitations patch user profiles](UserProfileGuardianInvitationPatchCall) (none)
/// * [guardians delete user profiles](UserProfileGuardianDeleteCall) (none)
/// * [guardians get user profiles](UserProfileGuardianGetCall) (none)
/// * [guardians list user profiles](UserProfileGuardianListCall) (none)
/// * [get user profiles](UserProfileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// Email address of the user. Must request `https://www.googleapis.com/auth/classroom.profile.emails` scope for this field to be populated in a response body. Read-only.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Identifier of the user. Read-only.
    
    pub id: Option<String>,
    /// Name of the user. Read-only.
    
    pub name: Option<Name>,
    /// Global permissions of the user. Read-only.
    
    pub permissions: Option<Vec<GlobalPermission>>,
    /// URL of user's profile photo. Must request `https://www.googleapis.com/auth/classroom.profile.photos` scope for this field to be populated in a response body. Read-only.
    #[serde(rename="photoUrl")]
    
    pub photo_url: Option<String>,
    /// Represents whether a Google Workspace for Education user's domain administrator has explicitly verified them as being a teacher. This field is always false if the user is not a member of a Google Workspace for Education domain. Read-only
    #[serde(rename="verifiedTeacher")]
    
    pub verified_teacher: Option<bool>,
}

impl client::Resource for UserProfile {}
impl client::ResponseResult for UserProfile {}


/// YouTube video item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YouTubeVideo {
    /// URL that can be used to view the YouTube video. Read-only.
    #[serde(rename="alternateLink")]
    
    pub alternate_link: Option<String>,
    /// YouTube API resource ID.
    
    pub id: Option<String>,
    /// URL of a thumbnail image of the YouTube video. Read-only.
    #[serde(rename="thumbnailUrl")]
    
    pub thumbnail_url: Option<String>,
    /// Title of the YouTube video. Read-only.
    
    pub title: Option<String>,
}

impl client::Part for YouTubeVideo {}


