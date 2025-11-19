use spectrum_analyzer::{
    FrequencyLimit, FrequencySpectrum, error::SpectrumAnalyzerError, samples_fft_to_spectrum,
    scaling::divide_by_N, windows::hann_window,
};

pub fn compute_spectrum(
    raw_data: &[f32],
    sample_rate: u32,
) -> Result<FrequencySpectrum, SpectrumAnalyzerError> {
    let window = hann_window(raw_data);
    let fft = samples_fft_to_spectrum(
        &window,
        sample_rate,
        FrequencyLimit::Range(20.0, 22050.0),
        Some(&divide_by_N),
    );
    fft
}
