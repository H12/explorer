// MiMalloc won´t compile on Windows with the GCC compiler.
// On Linux with Musl it won´t load correctly.
#[cfg(not(any(
    all(windows, target_env = "gnu"),
    all(target_os = "linux", target_env = "musl")
)))]
use mimalloc::MiMalloc;
use rustler::{Env, Term};

#[cfg(not(any(
    all(windows, target_env = "gnu"),
    all(target_os = "linux", target_env = "musl")
)))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod dataframe;
mod datatypes;
mod encoding;
mod error;
mod expressions;
mod lazyframe;
mod series;

use dataframe::io::*;
use dataframe::*;
pub use datatypes::{
    ExDataFrame, ExDataFrameRef, ExExpr, ExExprRef, ExLazyFrame, ExLazyFrameRef, ExSeries,
    ExSeriesRef,
};
pub use error::ExplorerError;
use expressions::*;
use lazyframe::io::*;
use lazyframe::*;
use series::log::*;
use series::*;

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExDataFrameRef, env);
    rustler::resource!(ExExprRef, env);
    rustler::resource!(ExLazyFrameRef, env);
    rustler::resource!(ExSeriesRef, env);
    true
}

mod atoms {
    rustler::atoms! {
        calendar_iso_module = "Elixir.Calendar.ISO",
        date_module = "Elixir.Date",
        naive_datetime_module = "Elixir.NaiveDateTime",
        time_module = "Elixir.Time",
        hour,
        minute,
        second,
        day,
        month,
        year,
        microsecond,
        calendar,
        nan,
        infinity,
        neg_infinity
    }
}

rustler::init!(
    "Elixir.Explorer.PolarsBackend.Native",
    [
        df_from_arrow_stream_pointer,
        df_arrange,
        df_arrange_with,
        df_concat_columns,
        df_concat_rows,
        df_describe,
        df_nil_count,
        df_distinct,
        df_drop,
        df_drop_nils,
        df_dtypes,
        df_dump_csv,
        df_dump_ndjson,
        df_dump_parquet,
        df_dump_ipc,
        df_dump_ipc_stream,
        df_filter_with,
        df_from_csv,
        df_from_ipc,
        df_from_ipc_stream,
        df_from_ndjson,
        df_from_parquet,
        df_from_series,
        df_group_indices,
        df_groups,
        df_head,
        df_join,
        df_load_csv,
        df_load_ndjson,
        df_load_parquet,
        df_load_ipc,
        df_load_ipc_stream,
        df_mask,
        df_mutate_with_exprs,
        df_n_rows,
        df_names,
        df_pivot_longer,
        df_pivot_wider,
        df_pull,
        df_put_column,
        df_rename_columns,
        df_sample_frac,
        df_sample_n,
        df_select,
        df_select_at_idx,
        df_shape,
        df_slice,
        df_slice_by_indices,
        df_slice_by_series,
        df_summarise_with_exprs,
        df_tail,
        df_to_csv,
        df_to_dummies,
        df_to_ipc,
        df_to_ipc_stream,
        df_to_lazy,
        df_to_ndjson,
        df_to_parquet,
        df_width,
        // expressions
        expr_atom,
        expr_boolean,
        expr_cast,
        expr_column,
        expr_date,
        expr_datetime,
        expr_day_of_week,
        expr_month,
        expr_year,
        expr_hour,
        expr_minute,
        expr_second,
        expr_strptime,
        expr_strftime,
        expr_clip_integer,
        expr_clip_float,
        expr_fill_missing_with_strategy,
        expr_fill_missing_with_value,
        expr_float,
        expr_head,
        expr_integer,
        expr_peaks,
        expr_rank,
        expr_unary_not,
        expr_sample_frac,
        expr_sample_n,
        expr_series,
        expr_shift,
        expr_slice,
        expr_slice_by_indices,
        expr_string,
        expr_tail,
        // sort
        expr_argsort,
        expr_distinct,
        expr_reverse,
        expr_sort,
        expr_unordered_distinct,
        // comparison expressions
        expr_all_equal,
        expr_binary_and,
        expr_binary_or,
        expr_binary_in,
        expr_equal,
        expr_greater,
        expr_greater_equal,
        expr_is_nil,
        expr_is_not_nil,
        expr_less,
        expr_less_equal,
        expr_not_equal,
        // arithmetic expressions
        expr_add,
        expr_abs,
        expr_divide,
        expr_multiply,
        expr_pow,
        expr_log,
        expr_log_natural,
        expr_exp,
        expr_quotient,
        expr_remainder,
        expr_subtract,
        // trigonometric expressions
        expr_acos,
        expr_asin,
        expr_atan,
        expr_cos,
        expr_sin,
        expr_tan,
        // slice and dice expressions
        expr_coalesce,
        expr_format,
        expr_concat,
        expr_select,
        // agg expressions
        expr_alias,
        expr_argmax,
        expr_argmin,
        expr_count,
        expr_first,
        expr_last,
        expr_max,
        expr_mean,
        expr_median,
        expr_min,
        expr_n_distinct,
        expr_nil_count,
        expr_quantile,
        expr_standard_deviation,
        expr_sum,
        expr_variance,
        expr_product,
        expr_skew,
        expr_correlation,
        expr_covariance,
        // window expressions
        expr_cumulative_max,
        expr_cumulative_min,
        expr_cumulative_sum,
        expr_cumulative_product,
        expr_window_max,
        expr_window_mean,
        expr_window_min,
        expr_window_sum,
        expr_window_standard_deviation,
        expr_ewm_mean,
        // inspect expressions
        expr_describe_filter_plan,
        // string expressions
        expr_contains,
        expr_upcase,
        expr_downcase,
        expr_trim,
        expr_trim_leading,
        expr_trim_trailing,
        // float round expressions
        expr_round,
        expr_floor,
        expr_ceil,
        // lazyframe
        lf_collect,
        lf_describe_plan,
        lf_drop,
        lf_dtypes,
        lf_fetch,
        lf_head,
        lf_names,
        lf_select,
        lf_tail,
        lf_slice,
        lf_from_csv,
        lf_from_ipc,
        lf_from_parquet,
        lf_from_ndjson,
        lf_filter_with,
        lf_arrange_with,
        lf_distinct,
        lf_mutate_with,
        lf_summarise_with,
        lf_rename_columns,
        lf_drop_nils,
        lf_pivot_longer,
        lf_join,
        lf_concat_rows,
        lf_concat_columns,
        lf_to_parquet,
        lf_to_ipc,
        // series
        s_as_str,
        s_abs,
        s_add,
        s_and,
        s_argmax,
        s_argmin,
        s_argsort,
        s_acos,
        s_asin,
        s_atan,
        s_cast,
        s_categories,
        s_categorise,
        s_coalesce,
        s_concat,
        s_contains,
        s_cos,
        s_upcase,
        s_day_of_week,
        s_month,
        s_year,
        s_hour,
        s_minute,
        s_second,
        s_strptime,
        s_strftime,
        s_clip_integer,
        s_clip_float,
        s_downcase,
        s_cumulative_max,
        s_cumulative_min,
        s_cumulative_sum,
        s_cumulative_product,
        s_distinct,
        s_divide,
        s_dtype,
        s_equal,
        s_exp,
        s_fill_missing_with_strategy,
        s_fill_missing_with_bin,
        s_fill_missing_with_boolean,
        s_fill_missing_with_float,
        s_fill_missing_with_int,
        s_fill_missing_with_atom,
        s_fill_missing_with_date,
        s_fill_missing_with_datetime,
        s_format,
        s_greater,
        s_greater_equal,
        s_head,
        s_is_not_null,
        s_is_null,
        s_is_finite,
        s_is_infinite,
        s_is_nan,
        s_less,
        s_less_equal,
        s_trim_leading,
        s_mask,
        s_max,
        s_mean,
        s_median,
        s_product,
        s_skew,
        s_correlation,
        s_covariance,
        s_min,
        s_multiply,
        s_n_distinct,
        s_name,
        s_nil_count,
        s_not,
        s_log,
        s_log_natural,
        s_from_list_bool,
        s_from_list_date,
        s_from_list_time,
        s_from_list_datetime,
        s_from_list_f64,
        s_from_list_i64,
        s_from_list_u32,
        s_from_list_str,
        s_from_list_binary,
        s_from_list_categories,
        s_from_binary_f64,
        s_from_binary_i64,
        s_from_binary_i32,
        s_from_binary_u8,
        s_not_equal,
        s_or,
        s_peak_max,
        s_peak_min,
        s_select,
        s_pow,
        s_pow_f_lhs,
        s_pow_f_rhs,
        s_pow_i_lhs,
        s_pow_i_rhs,
        s_quantile,
        s_quotient,
        s_rank,
        s_remainder,
        s_rename,
        s_reverse,
        s_trim_trailing,
        s_sample_n,
        s_sample_frac,
        s_series_equal,
        s_shift,
        s_sin,
        s_size,
        s_slice,
        s_slice_by_indices,
        s_slice_by_series,
        s_sort,
        s_standard_deviation,
        s_tan,
        s_trim,
        s_subtract,
        s_sum,
        s_tail,
        s_at,
        s_at_every,
        s_to_list,
        s_to_iovec,
        s_unordered_distinct,
        s_frequencies,
        s_cut,
        s_qcut,
        s_variance,
        s_window_max,
        s_window_mean,
        s_window_min,
        s_window_sum,
        s_window_standard_deviation,
        s_ewm_mean,
        s_in,
        s_round,
        s_floor,
        s_ceil,
    ],
    load = on_load
);
