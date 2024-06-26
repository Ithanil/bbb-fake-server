pub struct Meeting {
    // API Parameters according to https://bigbluebutton.org/api/
    pub name: String,
    pub meetingID: String,
    pub attendeePW: String,
    pub moderatorPW: String,
    pub welcome: String,
    pub dialNumber: String,
    pub voiceBridge: String,
    pub maxParticipants: uint,
    pub logoutURL: String,
    pub record: bool,
    pub duration: uint,
    pub isBreakout: bool,
    pub parentMeetingID: String,
    pub sequence: uint,
    pub freeJoin: bool,
    pub breakoutRoomsPrivateChatEnabled: bool,
    pub breakoutRoomsRecord: bool,
    pub meta: String,
    pub moderatorOnlyMessage: String,
    pub autoStartRecording: bool,
    pub allowStartStopRecording: bool,
    pub webcamsOnlyForModerator: bool,
    pub bannerText: String,
    pub bannerColor: String,
    pub muteOnStart: bool,
    pub allowModsToUnmuteUsers: bool,
    pub lockSettingsDisableCam: bool,
    pub lockSettingsDisableMic: bool,
    pub lockSettingsDisablePrivateChat: bool,
    pub lockSettingsDisablePublicChat: bool,
    pub lockSettingsDisableNotes: bool,
    pub lockSettingsLockOnJoin: bool,
    pub lockSettingsLockOnJoinConfigurable: bool,
    pub lockSettingsHideViewersCursor: bool,
    pub guestPolicy: String,
    pub meetingKeepEvents: bool,
    pub endWhenNoModerator: bool,
    pub endWhenNoModeratorDelayInMinutes: uint,
    pub meetingLayout: String,
    pub learningDashboardEnabled: bool,
    pub learningDashboardCleanupDelayInMinutes: uint,
    pub allowModsToEjectCameras: bool,
    pub allowRequestsWithoutSession: bool,
    pub userCameraCap: uint,
    pub meetingCameraCap: uint,
    pub meetingExpireIfNoUserJoinedInMinutes: uint,
    pub meetingExpireWhenLastUserLeftInMinutes: uint,
    pub groups: String,
    pub logo: String,
    pub disabledFeatures: String,
    pub disabledFeaturesExclude: String,
    pub preUploadedPresentationOverrideDefault: bool,
    pub notifyRecordingIsOn: bool,
    pub presentationUploadExternalUrl: String,
    pub presentationUploadExternalDescription: String,
    pub recordFullDurationMedia: bool,
    pub preUploadedPresentation: String,
    pub preUploadedPresentationName: String,

    // Internal Parameters
    pub createDate: String,
    pub attendees: Vec<String>
}
