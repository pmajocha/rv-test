fn main() {
    println!("cargo:rustc-link-lib=static=re2");
    println!("cargo:rustc-link-lib=static=absl_city");
    println!("cargo:rustc-link-lib=static=absl_hash");
    println!("cargo:rustc-link-lib=static=absl_raw_hash_set");
    println!("cargo:rustc-link-lib=static=absl_raw_logging_internal");
    println!("cargo:rustc-link-lib=static=absl_spinlock_wait");
    println!("cargo:rustc-link-lib=static=absl_str_format_internal");
    println!("cargo:rustc-link-lib=static=absl_synchronization");
    println!("cargo:rustc-link-lib=static=absl_low_level_hash");
    println!("cargo:rustc-link-lib=static=absl_strings");
    println!("cargo:rustc-link-lib=static=absl_strings_internal");
    println!("cargo:rustc-link-lib=static=absl_kernel_timeout_internal");
    println!("cargo:rustc-link-lib=static=absl_malloc_internal");
    println!("cargo:rustc-link-lib=static=absl_base");
    println!("cargo:rustc-link-lib=static=absl_stacktrace");
    println!("cargo:rustc-link-lib=static=absl_time");
    println!("cargo:rustc-link-lib=static=absl_civil_time");
    println!("cargo:rustc-link-lib=static=absl_cord");
    println!("cargo:rustc-link-lib=static=absl_cord_internal");
    println!("cargo:rustc-link-lib=static=absl_cordz_functions");
    println!("cargo:rustc-link-lib=static=absl_cordz_handle");
    println!("cargo:rustc-link-lib=static=absl_cordz_info");
    println!("cargo:rustc-link-lib=static=absl_cordz_sample_token");
    println!("cargo:rustc-link-lib=static=absl_crc32c");
    println!("cargo:rustc-link-lib=static=absl_crc_cord_state");
    println!("cargo:rustc-link-lib=static=absl_crc_cpu_detect");
    println!("cargo:rustc-link-lib=static=absl_crc_internal");
    println!("cargo:rustc-link-lib=static=absl_debugging_internal");
    println!("cargo:rustc-link-lib=static=absl_demangle_internal");
    println!("cargo:rustc-link-lib=static=absl_die_if_null");
    println!("cargo:rustc-link-lib=static=absl_examine_stack");
    println!("cargo:rustc-link-lib=static=absl_exponential_biased");
    println!("cargo:rustc-link-lib=static=absl_failure_signal_handler");
    println!("cargo:rustc-link-lib=static=absl_flags");
    println!("cargo:rustc-link-lib=static=absl_flags_commandlineflag");
    println!("cargo:rustc-link-lib=static=absl_flags_commandlineflag_internal");
    println!("cargo:rustc-link-lib=static=absl_flags_config");
    println!("cargo:rustc-link-lib=static=absl_flags_internal");
    println!("cargo:rustc-link-lib=static=absl_flags_marshalling");
    println!("cargo:rustc-link-lib=static=absl_flags_parse");
    println!("cargo:rustc-link-lib=static=absl_flags_private_handle_accessor");
    println!("cargo:rustc-link-lib=static=absl_flags_program_name");
    println!("cargo:rustc-link-lib=static=absl_flags_reflection");
    println!("cargo:rustc-link-lib=static=absl_flags_usage");
    println!("cargo:rustc-link-lib=static=absl_flags_usage_internal");
    println!("cargo:rustc-link-lib=static=absl_graphcycles_internal");
    println!("cargo:rustc-link-lib=static=absl_hashtablez_sampler");
    println!("cargo:rustc-link-lib=static=absl_int128");
    println!("cargo:rustc-link-lib=static=absl_leak_check");
    println!("cargo:rustc-link-lib=static=absl_log_entry");
    println!("cargo:rustc-link-lib=static=absl_log_flags");
    println!("cargo:rustc-link-lib=static=absl_log_globals");
    println!("cargo:rustc-link-lib=static=absl_log_initialize");
    println!("cargo:rustc-link-lib=static=absl_log_internal_check_op");
    println!("cargo:rustc-link-lib=static=absl_log_internal_conditions");
    println!("cargo:rustc-link-lib=static=absl_log_internal_format");
    println!("cargo:rustc-link-lib=static=absl_log_internal_globals");
    println!("cargo:rustc-link-lib=static=absl_log_internal_log_sink_set");
    println!("cargo:rustc-link-lib=static=absl_log_internal_message");
    println!("cargo:rustc-link-lib=static=absl_log_internal_nullguard");
    println!("cargo:rustc-link-lib=static=absl_log_internal_proto");
    println!("cargo:rustc-link-lib=static=absl_log_severity");
    println!("cargo:rustc-link-lib=static=absl_log_sink");
    println!("cargo:rustc-link-lib=static=absl_periodic_sampler");
    println!("cargo:rustc-link-lib=static=absl_random_distributions");
    println!("cargo:rustc-link-lib=static=absl_random_internal_distribution_test_util");
    println!("cargo:rustc-link-lib=static=absl_random_internal_platform");
    println!("cargo:rustc-link-lib=static=absl_random_internal_pool_urbg");
    println!("cargo:rustc-link-lib=static=absl_random_internal_randen");
    println!("cargo:rustc-link-lib=static=absl_random_internal_randen_hwaes");
    println!("cargo:rustc-link-lib=static=absl_random_internal_randen_hwaes_impl");
    println!("cargo:rustc-link-lib=static=absl_random_internal_randen_slow");
    println!("cargo:rustc-link-lib=static=absl_random_internal_seed_material");
    println!("cargo:rustc-link-lib=static=absl_random_seed_gen_exception");
    println!("cargo:rustc-link-lib=static=absl_random_seed_sequences");
    println!("cargo:rustc-link-lib=static=absl_scoped_set_env");
    println!("cargo:rustc-link-lib=static=absl_status");
    println!("cargo:rustc-link-lib=static=absl_statusor");
    println!("cargo:rustc-link-lib=static=absl_strerror");
    println!("cargo:rustc-link-lib=static=absl_string_view");
    println!("cargo:rustc-link-lib=static=absl_symbolize");
    println!("cargo:rustc-link-lib=static=absl_throw_delegate");
    println!("cargo:rustc-link-lib=static=absl_time_zone");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-search=./lib");
    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rerun-if-changed=build.rs");
    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp")
        .include("./include")
        .compile("wrapper");
}