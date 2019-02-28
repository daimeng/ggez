use crate::audio::SoundSource;
use crate::tests;
use crate::*;

#[test]
fn audio_load_ogg() {
    let (c, _e) = &mut tests::make_context();

    // OGG format
    let filename = "/pew.ogg";
    let _sound = audio::Source::new(c, filename).unwrap();
    let _sound = audio::SpatialSource::new(c, filename).unwrap();
}

#[test]
fn audio_load_wav() {
    let (c, _e) = &mut tests::make_context();

    // WAV format
    let filename = "/pew.wav";
    let _sound = audio::Source::new(c, filename).unwrap();
    let _sound = audio::SpatialSource::new(c, filename).unwrap();
}

#[test]
fn audio_load_flac() {
    let (c, _e) = &mut tests::make_context();

    // FLAC format
    let filename = "/pew.flac";
    let _sound = audio::Source::new(c, filename).unwrap();
    let _sound = audio::SpatialSource::new(c, filename).unwrap();
}

#[test]
fn fail_when_loading_nonexistent_file() {
    let (c, _e) = &mut tests::make_context();

    // File does not exist
    let filename = "/does-not-exist.ogg";
    assert!(audio::Source::new(c, filename).is_err());
    assert!(audio::SpatialSource::new(c, filename).is_err());
}

#[test]
fn fail_when_loading_non_audio_file() {
    let (c, _e) = &mut tests::make_context();

    let filename = "/player.png";
    assert!(audio::Source::new(c, filename).is_err());
    assert!(audio::SpatialSource::new(c, filename).is_err());
}

#[test]
fn volume_persists_after_stop() {
    let (c, _e) = &mut tests::make_context();
    let filename = "/pew.ogg";
    {
        let volume = 0.8;
        let mut sound = audio::Source::new(c, filename).unwrap();
        sound.set_volume(volume);
        assert_eq!(sound.volume(), volume);
        sound.stop();
        assert_eq!(sound.volume(), volume);
    }

    {
        let volume = 0.8;
        let mut sound = audio::SpatialSource::new(c, filename).unwrap();
        sound.set_volume(volume);
        assert_eq!(sound.volume(), volume);
        sound.stop();
        assert_eq!(sound.volume(), volume);
    }
}

#[test]
fn volume_persists_after_play() {
    let (c, _e) = &mut tests::make_context();
    let filename = "/pew.ogg";
    {
        let volume = 0.8;
        let mut sound = audio::Source::new(c, filename).unwrap();
        sound.set_volume(volume);
        assert_eq!(sound.volume(), volume);
        sound.play().unwrap();
        assert_eq!(sound.volume(), volume);
    }

    {
        let volume = 0.8;
        let mut sound = audio::SpatialSource::new(c, filename).unwrap();
        sound.set_volume(volume);
        assert_eq!(sound.volume(), volume);
        sound.play().unwrap();
        assert_eq!(sound.volume(), volume);
    }
}
