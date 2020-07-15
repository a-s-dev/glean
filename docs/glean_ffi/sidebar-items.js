initSidebarItems({"constant":[["UPLOAD_RESULT_HTTP_STATUS","A HTTP response code."],["UPLOAD_RESULT_RECOVERABLE","A recoverable error."],["UPLOAD_RESULT_UNRECOVERABLE","An unrecoverable error."]],"enum":[["Lifetime","The supported metrics' lifetimes."],["MemoryUnit","Different resolutions supported by the memory related metric types (e.g. MemoryDistributionMetric)."],["TimeUnit","Different resolutions supported by the time related metric types (e.g. DatetimeMetric)."]],"fn":[["glean_clear_application_lifetime_metrics",""],["glean_destroy_glean",""],["glean_enable_logging","Initialize the logging system based on the target platform. This ensures that logging is shown when executing the Glean SDK unit tests."],["glean_experiment_test_get_data",""],["glean_experiment_test_is_active",""],["glean_get_upload_task",""],["glean_initialize","Safety"],["glean_initialize_for_subprocess","Safety"],["glean_is_dirty_flag_set",""],["glean_is_first_run",""],["glean_is_upload_enabled",""],["glean_on_ready_to_submit_pings",""],["glean_ping_collect",""],["glean_process_ping_upload_response","Process and free a `FfiPingUploadTask`."],["glean_set_debug_view_tag",""],["glean_set_dirty_flag",""],["glean_set_experiment_active",""],["glean_set_experiment_inactive",""],["glean_set_log_pings",""],["glean_set_upload_enabled",""],["glean_str_free","Public destructor for strings managed by the other side of the FFI."],["glean_submit_ping_by_name",""],["glean_test_clear_all_stores",""]],"macro":[["define_infallible_handle_map_deleter",""],["define_metric","Define the global handle map, constructor and destructor functions and any user-defined functions for a new metric"]],"mod":[["byte_buffer","ByteBuffer is a struct that represents an array of bytes to be sent over the FFI boundaries."],["ping_type",""],["upload","FFI compatible types for the upload mechanism."]],"struct":[["FfiConfiguration","Configuration over FFI."]]});