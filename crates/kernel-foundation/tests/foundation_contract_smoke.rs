use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, Clock, CorrelationId, JobId, PageCursor,
    PageRequest, PageSlice, SystemClock,
};

#[test]
fn foundation_contract_smoke() {
    let correlation_id = CorrelationId::generate();
    let job_id = JobId::generate();
    let page_request = PageRequest::new(25, Some(PageCursor::new("cursor-1")));
    let page = PageSlice::new(vec![job_id.clone()], Some(PageCursor::new("cursor-2")));

    assert_eq!(page_request.limit, 25);
    assert_eq!(page_request.cursor.as_ref().map(PageCursor::as_str), Some("cursor-1"));

    let encoded_page = serde_json::to_string(&page).expect("page slice should serialize");
    let decoded_page: PageSlice<JobId> =
        serde_json::from_str(&encoded_page).expect("page slice should deserialize");
    assert_eq!(decoded_page, page);

    let app_error = AppError::new(
        "LIB_SMOKE",
        "foundation smoke",
        true,
        AppErrorSeverity::Error,
        correlation_id.clone(),
    );
    let result: AppResult<PageSlice<JobId>> = Err(app_error.clone());

    assert!(result.is_err());
    assert_eq!(app_error.code, "LIB_SMOKE");
    assert_eq!(app_error.correlation_id, correlation_id);

    let now = SystemClock.now();
    assert!(!now.to_string().is_empty());
    assert_eq!(job_id.to_string(), job_id.as_str());
}