use super::*;
/// A builder providing access to all methods supported on *course* resources.
/// It is not used directly, but through the [`Classroom`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_classroom1 as classroom1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aliases_create(...)`, `aliases_delete(...)`, `aliases_list(...)`, `announcements_create(...)`, `announcements_delete(...)`, `announcements_get(...)`, `announcements_list(...)`, `announcements_modify_assignees(...)`, `announcements_patch(...)`, `course_work_create(...)`, `course_work_delete(...)`, `course_work_get(...)`, `course_work_list(...)`, `course_work_materials_create(...)`, `course_work_materials_delete(...)`, `course_work_materials_get(...)`, `course_work_materials_list(...)`, `course_work_materials_patch(...)`, `course_work_modify_assignees(...)`, `course_work_patch(...)`, `course_work_student_submissions_get(...)`, `course_work_student_submissions_list(...)`, `course_work_student_submissions_modify_attachments(...)`, `course_work_student_submissions_patch(...)`, `course_work_student_submissions_reclaim(...)`, `course_work_student_submissions_return(...)`, `course_work_student_submissions_turn_in(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)`, `students_create(...)`, `students_delete(...)`, `students_get(...)`, `students_list(...)`, `teachers_create(...)`, `teachers_delete(...)`, `teachers_get(...)`, `teachers_list(...)`, `topics_create(...)`, `topics_delete(...)`, `topics_get(...)`, `topics_list(...)`, `topics_patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.courses();
/// # }
/// ```
pub struct CourseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Classroom<S>,
}

impl<'a, S> client::MethodsBuilder for CourseMethods<'a, S> {}

impl<'a, S> CourseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an alias for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create the alias or for access errors. * `NOT_FOUND` if the course does not exist. * `ALREADY_EXISTS` if the alias already exists. * `FAILED_PRECONDITION` if the alias requested does not make sense for the requesting user or course (for example, if a user not in a domain attempts to access a domain-scoped alias).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course to alias. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn aliases_create(&self, request: CourseAlias, course_id: &str) -> CourseAliasCreateCall<'a, S> {
        CourseAliasCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an alias of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to remove the alias or for access errors. * `NOT_FOUND` if the alias does not exist. * `FAILED_PRECONDITION` if the alias requested does not make sense for the requesting user or course (for example, if a user not in a domain attempts to delete a domain-scoped alias).
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course whose alias should be deleted. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `alias` - Alias to delete. This may not be the Classroom-assigned identifier.
    pub fn aliases_delete(&self, course_id: &str, alias: &str) -> CourseAliasDeleteCall<'a, S> {
        CourseAliasDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _alias: alias.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of aliases for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the course or for access errors. * `NOT_FOUND` if the course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - The identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn aliases_list(&self, course_id: &str) -> CourseAliasListCall<'a, S> {
        CourseAliasListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create announcements in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn announcements_create(&self, request: Announcement, course_id: &str) -> CourseAnnouncementCreateCall<'a, S> {
        CourseAnnouncementCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an announcement. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding announcement item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding announcement, if the requesting user is not permitted to delete the requested course or for access errors. * `FAILED_PRECONDITION` if the requested announcement has already been deleted. * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the announcement to delete. This identifier is a Classroom-assigned identifier.
    pub fn announcements_delete(&self, course_id: &str, id: &str) -> CourseAnnouncementDeleteCall<'a, S> {
        CourseAnnouncementDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or announcement, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or announcement does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the announcement.
    pub fn announcements_get(&self, course_id: &str, id: &str) -> CourseAnnouncementGetCall<'a, S> {
        CourseAnnouncementGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of announcements that the requester is permitted to view. Course students may only view `PUBLISHED` announcements. Course teachers and domain administrators may view all announcements. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn announcements_list(&self, course_id: &str) -> CourseAnnouncementListCall<'a, S> {
        CourseAnnouncementListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _announcement_states: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies assignee mode and options of an announcement. Only a teacher of the course that contains the announcement may call this method. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the announcement.
    pub fn announcements_modify_assignees(&self, request: ModifyAnnouncementAssigneesRequest, course_id: &str, id: &str) -> CourseAnnouncementModifyAssigneeCall<'a, S> {
        CourseAnnouncementModifyAssigneeCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding announcement or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if the requested announcement has already been deleted. * `NOT_FOUND` if the requested course or announcement does not exist
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the announcement.
    pub fn announcements_patch(&self, request: Announcement, course_id: &str, id: &str) -> CourseAnnouncementPatchCall<'a, S> {
        CourseAnnouncementPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student submission. * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, course work, or student submission or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_get(&self, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionGetCall<'a, S> {
        CourseCourseWorkStudentSubmissionGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of student submissions that the requester is permitted to view, factoring in the OAuth scopes of the request. `-` may be specified as the `course_work_id` to include student submissions for multiple course work items. Course students may only view their own work. Course teachers and domain administrators may view all student submissions. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the student work to request. This may be set to the string literal `"-"` to request student work for all course work in the specified course.
    pub fn course_work_student_submissions_list(&self, course_id: &str, course_work_id: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, S> {
        CourseCourseWorkStudentSubmissionListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _user_id: Default::default(),
            _states: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _late: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies attachments of student submission. Attachments may only be added to student submissions belonging to course work objects with a `workType` of `ASSIGNMENT`. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, if the user is not permitted to modify attachments on the requested student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_modify_attachments(&self, request: ModifyAttachmentsRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, S> {
        CourseCourseWorkStudentSubmissionModifyAttachmentCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a student submission. See google.classroom.v1.StudentSubmission for details of which fields may be updated and who may change them. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work, if the user is not permitted to make the requested modification to the student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_patch(&self, request: StudentSubmission, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, S> {
        CourseCourseWorkStudentSubmissionPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reclaims a student submission on behalf of the student that owns it. Reclaiming a student submission transfers ownership of attached Drive files to the student and updates the submission state. Only the student that owns the requested student submission may call this method, and only for a student submission that has been turned in. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, unsubmit the requested student submission, or for access errors. * `FAILED_PRECONDITION` if the student submission has not been turned in. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_reclaim(&self, request: ReclaimStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, S> {
        CourseCourseWorkStudentSubmissionReclaimCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student submission. Returning a student submission transfers ownership of attached Drive files to the student and may also update the submission state. Unlike the Classroom application, returning a student submission does not set assignedGrade to the draftGrade value. Only a teacher of the course that contains the requested student submission may call this method. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, return the requested student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_return(&self, request: ReturnStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionReturnCall<'a, S> {
        CourseCourseWorkStudentSubmissionReturnCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Turns in a student submission. Turning in a student submission transfers ownership of attached Drive files to the teacher and may also update the submission state. This may only be called by the student that owns the specified student submission. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, turn in the requested student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_turn_in(&self, request: TurnInStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, S> {
        CourseCourseWorkStudentSubmissionTurnInCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates course work. The resulting course work (and corresponding student submissions) are associated with the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to make the request. Classroom API requests to modify course work and student submissions must be made with an OAuth client ID from the associated Developer Console project. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create course work in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn course_work_create(&self, request: CourseWork, course_id: &str) -> CourseCourseWorkCreateCall<'a, S> {
        CourseCourseWorkCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a course work. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work, if the requesting user is not permitted to delete the requested course or for access errors. * `FAILED_PRECONDITION` if the requested course work has already been deleted. * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work to delete. This identifier is a Classroom-assigned identifier.
    pub fn course_work_delete(&self, course_id: &str, id: &str) -> CourseCourseWorkDeleteCall<'a, S> {
        CourseCourseWorkDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns course work. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work.
    pub fn course_work_get(&self, course_id: &str, id: &str) -> CourseCourseWorkGetCall<'a, S> {
        CourseCourseWorkGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of course work that the requester is permitted to view. Course students may only view `PUBLISHED` course work. Course teachers and domain administrators may view all course work. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn course_work_list(&self, course_id: &str) -> CourseCourseWorkListCall<'a, S> {
        CourseCourseWorkListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _course_work_states: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies assignee mode and options of a coursework. Only a teacher of the course that contains the coursework may call this method. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the coursework.
    pub fn course_work_modify_assignees(&self, request: ModifyCourseWorkAssigneesRequest, course_id: &str, id: &str) -> CourseCourseWorkModifyAssigneeCall<'a, S> {
        CourseCourseWorkModifyAssigneeCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a course work. See google.classroom.v1.CourseWork for details of which fields may be updated and who may change them. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work, if the user is not permitted to make the requested modification to the student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if the requested course work has already been deleted. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work.
    pub fn course_work_patch(&self, request: CourseWork, course_id: &str, id: &str) -> CourseCourseWorkPatchCall<'a, S> {
        CourseCourseWorkPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create course work material in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed or if more than 20 * materials are provided. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn course_work_materials_create(&self, request: CourseWorkMaterial, course_id: &str) -> CourseCourseWorkMaterialCreateCall<'a, S> {
        CourseCourseWorkMaterialCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a course work material. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work material item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work material, if the requesting user is not permitted to delete the requested course or for access errors. * `FAILED_PRECONDITION` if the requested course work material has already been deleted. * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work material to delete. This identifier is a Classroom-assigned identifier.
    pub fn course_work_materials_delete(&self, course_id: &str, id: &str) -> CourseCourseWorkMaterialDeleteCall<'a, S> {
        CourseCourseWorkMaterialDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work material, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work material does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work material.
    pub fn course_work_materials_get(&self, course_id: &str, id: &str) -> CourseCourseWorkMaterialGetCall<'a, S> {
        CourseCourseWorkMaterialGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of course work material that the requester is permitted to view. Course students may only view `PUBLISHED` course work material. Course teachers and domain administrators may view all course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn course_work_materials_list(&self, course_id: &str) -> CourseCourseWorkMaterialListCall<'a, S> {
        CourseCourseWorkMaterialListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _material_link: Default::default(),
            _material_drive_id: Default::default(),
            _course_work_material_states: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if the requested course work material has already been deleted. * `NOT_FOUND` if the requested course or course work material does not exist
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the course work material.
    pub fn course_work_materials_patch(&self, request: CourseWorkMaterial, course_id: &str, id: &str) -> CourseCourseWorkMaterialPatchCall<'a, S> {
        CourseCourseWorkMaterialPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a user as a student of a course. Domain administrators are permitted to [directly add](https://developers.google.com/classroom/guides/manage-users) users within their domain as students to courses within their domain. Students are permitted to add themselves to a course using an enrollment code. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create students in this course or for access errors. * `NOT_FOUND` if the requested course ID does not exist. * `FAILED_PRECONDITION` if the requested user's account is disabled, for the following request errors: * CourseMemberLimitReached * CourseNotModifiable * UserGroupsMembershipLimitReached * InactiveCourseOwner * `ALREADY_EXISTS` if the user is already a student or teacher in the course.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course to create the student in. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn students_create(&self, request: Student, course_id: &str) -> CourseStudentCreateCall<'a, S> {
        CourseStudentCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _enrollment_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a student of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete students of this course or for access errors. * `NOT_FOUND` if no student of this course has the requested ID or if the course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `userId` - Identifier of the student to delete. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    pub fn students_delete(&self, course_id: &str, user_id: &str) -> CourseStudentDeleteCall<'a, S> {
        CourseStudentDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view students of this course or for access errors. * `NOT_FOUND` if no student of this course has the requested ID or if the course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `userId` - Identifier of the student to return. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    pub fn students_get(&self, course_id: &str, user_id: &str) -> CourseStudentGetCall<'a, S> {
        CourseStudentGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of students of this course that the requester is permitted to view. This method returns the following error codes: * `NOT_FOUND` if the course does not exist. * `PERMISSION_DENIED` for access errors.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn students_list(&self, course_id: &str) -> CourseStudentListCall<'a, S> {
        CourseStudentListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a teacher of a course. Domain administrators are permitted to [directly add](https://developers.google.com/classroom/guides/manage-users) users within their domain as teachers to courses within their domain. Non-admin users should send an Invitation instead. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create teachers in this course or for access errors. * `NOT_FOUND` if the requested course ID does not exist. * `FAILED_PRECONDITION` if the requested user's account is disabled, for the following request errors: * CourseMemberLimitReached * CourseNotModifiable * CourseTeacherLimitReached * UserGroupsMembershipLimitReached * InactiveCourseOwner * `ALREADY_EXISTS` if the user is already a teacher or student in the course.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn teachers_create(&self, request: Teacher, course_id: &str) -> CourseTeacherCreateCall<'a, S> {
        CourseTeacherCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified teacher from the specified course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete teachers of this course or for access errors. * `NOT_FOUND` if no teacher of this course has the requested ID or if the course does not exist. * `FAILED_PRECONDITION` if the requested ID belongs to the primary teacher of this course. * `FAILED_PRECONDITION` if the requested ID belongs to the owner of the course Drive folder. * `FAILED_PRECONDITION` if the course no longer has an active owner.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `userId` - Identifier of the teacher to delete. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    pub fn teachers_delete(&self, course_id: &str, user_id: &str) -> CourseTeacherDeleteCall<'a, S> {
        CourseTeacherDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a teacher of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view teachers of this course or for access errors. * `NOT_FOUND` if no teacher of this course has the requested ID or if the course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `userId` - Identifier of the teacher to return. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    pub fn teachers_get(&self, course_id: &str, user_id: &str) -> CourseTeacherGetCall<'a, S> {
        CourseTeacherGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of teachers of this course that the requester is permitted to view. This method returns the following error codes: * `NOT_FOUND` if the course does not exist. * `PERMISSION_DENIED` for access errors.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn teachers_list(&self, course_id: &str) -> CourseTeacherListCall<'a, S> {
        CourseTeacherListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create a topic in the requested course, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn topics_create(&self, request: Topic, course_id: &str) -> CourseTopicCreateCall<'a, S> {
        CourseTopicCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not allowed to delete the requested topic or for access errors. * `FAILED_PRECONDITION` if the requested topic has already been deleted. * `NOT_FOUND` if no course or topic exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the topic to delete.
    pub fn topics_delete(&self, course_id: &str, id: &str) -> CourseTopicDeleteCall<'a, S> {
        CourseTopicDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or topic, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or topic does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    /// * `id` - Identifier of the topic.
    pub fn topics_get(&self, course_id: &str, id: &str) -> CourseTopicGetCall<'a, S> {
        CourseTopicGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of topics that the requester is permitted to view. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn topics_list(&self, course_id: &str) -> CourseTopicListCall<'a, S> {
        CourseTopicListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding topic or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or topic does not exist
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    /// * `id` - Identifier of the topic.
    pub fn topics_patch(&self, request: Topic, course_id: &str, id: &str) -> CourseTopicPatchCall<'a, S> {
        CourseTopicPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a course. The user specified in `ownerId` is the owner of the created course and added as a teacher. A non-admin requesting user can only create a course with themselves as the owner. Domain admins can create courses owned by any user within their domain. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create courses or for access errors. * `NOT_FOUND` if the primary teacher is not a valid user. * `FAILED_PRECONDITION` if the course owner's account is disabled or for the following request errors: * UserCannotOwnCourse * UserGroupsMembershipLimitReached * `ALREADY_EXISTS` if an alias was specified in the `id` and already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Course) -> CourseCreateCall<'a, S> {
        CourseCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the course to delete. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn delete(&self, id: &str) -> CourseDeleteCall<'a, S> {
        CourseDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the course to return. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn get(&self, id: &str) -> CourseGetCall<'a, S> {
        CourseGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of courses that the requesting user is permitted to view, restricted to those that match the request. Returned courses are ordered by creation time, with the most recently created coming first. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the query argument is malformed. * `NOT_FOUND` if any users specified in the query arguments do not exist.
    pub fn list(&self) -> CourseListCall<'a, S> {
        CourseListCall {
            hub: self.hub,
            _teacher_id: Default::default(),
            _student_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _course_states: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields in a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to modify the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID. * `INVALID_ARGUMENT` if invalid fields are specified in the update mask or if no update mask is supplied. * `FAILED_PRECONDITION` for the following request errors: * CourseNotModifiable * InactiveCourseOwner * IneligibleOwner
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier of the course to update. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn patch(&self, request: Course, id: &str) -> CoursePatchCall<'a, S> {
        CoursePatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to modify the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID. * `FAILED_PRECONDITION` for the following request errors: * CourseNotModifiable
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier of the course to update. This identifier can be either the Classroom-assigned identifier or an alias.
    pub fn update(&self, request: Course, id: &str) -> CourseUpdateCall<'a, S> {
        CourseUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *invitation* resources.
/// It is not used directly, but through the [`Classroom`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_classroom1 as classroom1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `create(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.invitations();
/// # }
/// ```
pub struct InvitationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Classroom<S>,
}

impl<'a, S> client::MethodsBuilder for InvitationMethods<'a, S> {}

impl<'a, S> InvitationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts an invitation, removing it and adding the invited user to the teachers or students (as appropriate) of the specified course. Only the invited user may accept an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to accept the requested invitation or for access errors. * `FAILED_PRECONDITION` for the following request errors: * CourseMemberLimitReached * CourseNotModifiable * CourseTeacherLimitReached * UserGroupsMembershipLimitReached * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to accept.
    pub fn accept(&self, id: &str) -> InvitationAcceptCall<'a, S> {
        InvitationAcceptCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an invitation. Only one invitation for a user and course may exist at a time. Delete and re-create an invitation to make changes. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create invitations for this course or for access errors. * `NOT_FOUND` if the course or the user does not exist. * `FAILED_PRECONDITION`: * if the requested user's account is disabled. * if the user already has this role or a role with greater permissions. * for the following request errors: * IneligibleOwner * `ALREADY_EXISTS` if an invitation for the specified user and course already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Invitation) -> InvitationCreateCall<'a, S> {
        InvitationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested invitation or for access errors. * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to delete.
    pub fn delete(&self, id: &str) -> InvitationDeleteCall<'a, S> {
        InvitationDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view the requested invitation or for access errors. * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to return.
    pub fn get(&self, id: &str) -> InvitationGetCall<'a, S> {
        InvitationGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of invitations that the requesting user is permitted to view, restricted to those that match the list request. *Note:* At least one of `user_id` or `course_id` must be supplied. Both fields can be supplied. This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    pub fn list(&self) -> InvitationListCall<'a, S> {
        InvitationListCall {
            hub: self.hub,
            _user_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _course_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *registration* resources.
/// It is not used directly, but through the [`Classroom`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_classroom1 as classroom1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)` and `delete(...)`
/// // to build up your call.
/// let rb = hub.registrations();
/// # }
/// ```
pub struct RegistrationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Classroom<S>,
}

impl<'a, S> client::MethodsBuilder for RegistrationMethods<'a, S> {}

impl<'a, S> RegistrationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a `Registration`, causing Classroom to start sending notifications from the provided `feed` to the destination provided in `cloudPubSubTopic`. Returns the created `Registration`. Currently, this will be the same as the argument, but with server-assigned fields such as `expiry_time` and `id` filled in. Note that any value specified for the `expiry_time` or `id` fields will be ignored. While Classroom may validate the `cloudPubSubTopic` and return errors on a best effort basis, it is the caller's responsibility to ensure that it exists and that Classroom has permission to publish to it. This method may return the following error codes: * `PERMISSION_DENIED` if: * the authenticated user does not have permission to receive notifications from the requested field; or * the current user has not granted access to the current Cloud project with the appropriate scope for the requested feed. Note that domain-wide delegation of authority is not currently supported for this purpose. If the request has the appropriate scope, but no grant exists, a Request Errors is returned. * another access error is encountered. * `INVALID_ARGUMENT` if: * no `cloudPubsubTopic` is specified, or the specified `cloudPubsubTopic` is not valid; or * no `feed` is specified, or the specified `feed` is not valid. * `NOT_FOUND` if: * the specified `feed` cannot be located, or the requesting user does not have permission to determine whether or not it exists; or * the specified `cloudPubsubTopic` cannot be located, or Classroom has not been granted permission to publish to it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Registration) -> RegistrationCreateCall<'a, S> {
        RegistrationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `Registration`, causing Classroom to stop sending notifications for that `Registration`.
    /// 
    /// # Arguments
    ///
    /// * `registrationId` - The `registration_id` of the `Registration` to be deleted.
    pub fn delete(&self, registration_id: &str) -> RegistrationDeleteCall<'a, S> {
        RegistrationDeleteCall {
            hub: self.hub,
            _registration_id: registration_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userProfile* resources.
/// It is not used directly, but through the [`Classroom`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_classroom1 as classroom1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `guardian_invitations_create(...)`, `guardian_invitations_get(...)`, `guardian_invitations_list(...)`, `guardian_invitations_patch(...)`, `guardians_delete(...)`, `guardians_get(...)` and `guardians_list(...)`
/// // to build up your call.
/// let rb = hub.user_profiles();
/// # }
/// ```
pub struct UserProfileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Classroom<S>,
}

impl<'a, S> client::MethodsBuilder for UserProfileMethods<'a, S> {}

impl<'a, S> UserProfileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a guardian invitation, and sends an email to the guardian asking them to confirm that they are the student's guardian. Once the guardian accepts the invitation, their `state` will change to `COMPLETED` and they will start receiving guardian notifications. A `Guardian` resource will also be created to represent the active guardian. The request object must have the `student_id` and `invited_email_address` fields set. Failing to set these fields, or setting any other fields in the request, will result in an error. This method returns the following error codes: * `PERMISSION_DENIED` if the current user does not have permission to manage guardians, if the guardian in question has already rejected too many requests for that student, if guardians are not enabled for the domain in question, or for other access errors. * `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian link limit. * `INVALID_ARGUMENT` if the guardian email address is not valid (for example, if it is too long), or if the format of the student ID provided cannot be recognized (it is not an email address, nor a `user_id` from this API). This error will also be returned if read-only fields are set, or if the `state` field is set to to a value other than `PENDING`. * `NOT_FOUND` if the student ID provided is a valid student ID, but Classroom has no record of that student. * `ALREADY_EXISTS` if there is already a pending guardian invitation for the student and `invited_email_address` provided, or if the provided `invited_email_address` matches the Google account of an existing `Guardian` for this user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `studentId` - ID of the student (in standard format)
    pub fn guardian_invitations_create(&self, request: GuardianInvitation, student_id: &str) -> UserProfileGuardianInvitationCreateCall<'a, S> {
        UserProfileGuardianInvitationCreateCall {
            hub: self.hub,
            _request: request,
            _student_id: student_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specific guardian invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view guardian invitations for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`). * `NOT_FOUND` if Classroom cannot find any record of the given student or `invitation_id`. May also be returned if the student exists, but the requesting user does not have access to see that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The ID of the student whose guardian invitation is being requested.
    /// * `invitationId` - The `id` field of the `GuardianInvitation` being requested.
    pub fn guardian_invitations_get(&self, student_id: &str, invitation_id: &str) -> UserProfileGuardianInvitationGetCall<'a, S> {
        UserProfileGuardianInvitationGetCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _invitation_id: invitation_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of guardian invitations that the requesting user is permitted to view, filtered by the parameters provided. This method returns the following error codes: * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting user is not permitted to view guardian invitations for that student, if `"-"` is specified as the `student_id` and the user is not a domain administrator, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`). May also be returned if an invalid `page_token` or `state` is provided. * `NOT_FOUND` if a `student_id` is specified, and its format can be recognized, but Classroom has no record of that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The ID of the student whose guardian invitations are to be returned. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user * the string literal `"-"`, indicating that results should be returned for all students that the requesting user is permitted to view guardian invitations.
    pub fn guardian_invitations_list(&self, student_id: &str) -> UserProfileGuardianInvitationListCall<'a, S> {
        UserProfileGuardianInvitationListCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _states: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _invited_email_address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a guardian invitation. Currently, the only valid modification is to change the `state` from `PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the current user does not have permission to manage guardians, if guardians are not enabled for the domain in question or for other access errors. * `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state. * `INVALID_ARGUMENT` if the format of the student ID provided cannot be recognized (it is not an email address, nor a `user_id` from this API), or if the passed `GuardianInvitation` has a `state` other than `COMPLETE`, or if it modifies fields other than `state`. * `NOT_FOUND` if the student ID provided is a valid student ID, but Classroom has no record of that student, or if the `id` field does not refer to a guardian invitation known to Classroom.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `studentId` - The ID of the student whose guardian invitation is to be modified.
    /// * `invitationId` - The `id` field of the `GuardianInvitation` to be modified.
    pub fn guardian_invitations_patch(&self, request: GuardianInvitation, student_id: &str, invitation_id: &str) -> UserProfileGuardianInvitationPatchCall<'a, S> {
        UserProfileGuardianInvitationPatchCall {
            hub: self.hub,
            _request: request,
            _student_id: student_id.to_string(),
            _invitation_id: invitation_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a guardian. The guardian will no longer receive guardian notifications and the guardian will no longer be accessible via the API. This method returns the following error codes: * `PERMISSION_DENIED` if no user that matches the provided `student_id` is visible to the requesting user, if the requesting user is not permitted to manage guardians for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API). * `NOT_FOUND` if the requesting user is permitted to modify guardians for the requested `student_id`, but no `Guardian` record exists for that student with the provided `guardian_id`.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The student whose guardian is to be deleted. One of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    /// * `guardianId` - The `id` field from a `Guardian`.
    pub fn guardians_delete(&self, student_id: &str, guardian_id: &str) -> UserProfileGuardianDeleteCall<'a, S> {
        UserProfileGuardianDeleteCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _guardian_id: guardian_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specific guardian. This method returns the following error codes: * `PERMISSION_DENIED` if no user that matches the provided `student_id` is visible to the requesting user, if the requesting user is not permitted to view guardian information for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`). * `NOT_FOUND` if the requesting user is permitted to view guardians for the requested `student_id`, but no `Guardian` record exists for that student that matches the provided `guardian_id`.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The student whose guardian is being requested. One of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    /// * `guardianId` - The `id` field from a `Guardian`.
    pub fn guardians_get(&self, student_id: &str, guardian_id: &str) -> UserProfileGuardianGetCall<'a, S> {
        UserProfileGuardianGetCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _guardian_id: guardian_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of guardians that the requesting user is permitted to view, restricted to those that match the request. To list guardians for any student that the requesting user may view guardians for, use the literal character `-` for the student ID. This method returns the following error codes: * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting user is not permitted to view guardian information for that student, if `"-"` is specified as the `student_id` and the user is not a domain administrator, if guardians are not enabled for the domain in question, if the `invited_email_address` filter is set by a user who is not a domain administrator, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`). May also be returned if an invalid `page_token` is provided. * `NOT_FOUND` if a `student_id` is specified, and its format can be recognized, but Classroom has no record of that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - Filter results by the student who the guardian is linked to. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user * the string literal `"-"`, indicating that results should be returned for all students that the requesting user has access to view.
    pub fn guardians_list(&self, student_id: &str) -> UserProfileGuardianListCall<'a, S> {
        UserProfileGuardianListCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _invited_email_address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a user profile. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access this user profile, if no profile exists with the requested ID, or for access errors.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Identifier of the profile to return. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user
    pub fn get(&self, user_id: &str) -> UserProfileGetCall<'a, S> {
        UserProfileGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



