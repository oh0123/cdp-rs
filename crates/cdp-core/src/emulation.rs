use crate::error::Result;
use crate::page::Page;
use cdp_protocol::emulation::{
    ClearGeolocationOverride, ClearGeolocationOverrideReturnObject, MediaFeature, SetEmulatedMedia,
    SetEmulatedMediaReturnObject, SetGeolocationOverride, SetGeolocationOverrideReturnObject,
    SetLocaleOverride, SetLocaleOverrideReturnObject, SetTimezoneOverride,
    SetTimezoneOverrideReturnObject, SetUserAgentOverride, SetUserAgentOverrideReturnObject,
    UserAgentBrandVersion, UserAgentMetadata,
};
use std::sync::Arc;

/// A group of emulation overrides applied to a target.
#[derive(Clone, Debug, Default)]
pub struct EmulationConfig {
    pub geolocation: Option<Geolocation>,
    pub timezone_id: Option<String>,
    pub locale: Option<String>,
    pub media: Option<MediaEmulation>,
    pub user_agent: Option<UserAgentOverride>,
}

impl EmulationConfig {
    pub fn with_geolocation(mut self, geolocation: Geolocation) -> Self {
        self.geolocation = Some(geolocation);
        self
    }

    pub fn with_timezone<T: Into<String>>(mut self, timezone: T) -> Self {
        self.timezone_id = Some(timezone.into());
        self
    }

    pub fn with_locale<T: Into<String>>(mut self, locale: T) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn with_media(mut self, media: MediaEmulation) -> Self {
        self.media = Some(media);
        self
    }

    pub fn with_user_agent(mut self, override_data: UserAgentOverride) -> Self {
        self.user_agent = Some(override_data);
        self
    }
}

/// Geolocation parameters for `Emulation.setGeolocationOverride`.
#[derive(Clone, Debug)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>,
    pub altitude: Option<f64>,
    pub altitude_accuracy: Option<f64>,
    pub heading: Option<f64>,
    pub speed: Option<f64>,
}

impl Geolocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            accuracy: None,
            altitude: None,
            altitude_accuracy: None,
            heading: None,
            speed: None,
        }
    }

    pub fn with_accuracy(mut self, accuracy: f64) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    pub fn with_altitude(mut self, altitude: f64) -> Self {
        self.altitude = Some(altitude);
        self
    }

    pub fn with_altitude_accuracy(mut self, altitude_accuracy: f64) -> Self {
        self.altitude_accuracy = Some(altitude_accuracy);
        self
    }

    pub fn with_heading(mut self, heading: f64) -> Self {
        self.heading = Some(heading);
        self
    }

    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }
}

impl Default for Geolocation {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// Media emulation configuration.
#[derive(Clone, Debug, Default)]
pub struct MediaEmulation {
    pub media_type: Option<String>,
    pub features: Vec<MediaFeatureOverride>,
}

impl MediaEmulation {
    pub fn with_media_type<T: Into<String>>(mut self, media: T) -> Self {
        self.media_type = Some(media.into());
        self
    }

    pub fn with_feature(mut self, feature: MediaFeatureOverride) -> Self {
        self.features.push(feature);
        self
    }
}

/// A single media feature override.
#[derive(Clone, Debug)]
pub struct MediaFeatureOverride {
    pub name: String,
    pub value: String,
}

impl MediaFeatureOverride {
    pub fn new<N: Into<String>, V: Into<String>>(name: N, value: V) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// High level user agent override description.
#[derive(Clone, Debug)]
pub struct UserAgentOverride {
    pub user_agent: String,
    pub accept_language: Option<String>,
    pub platform: Option<String>,
    pub metadata: Option<UserAgentMetadataOverride>,
}

impl UserAgentOverride {
    pub fn new<T: Into<String>>(user_agent: T) -> Self {
        Self {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            metadata: None,
        }
    }

    pub fn with_accept_language<T: Into<String>>(mut self, value: T) -> Self {
        self.accept_language = Some(value.into());
        self
    }

    pub fn with_platform<T: Into<String>>(mut self, value: T) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn with_metadata(mut self, metadata: UserAgentMetadataOverride) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Structured metadata for `SetUserAgentOverride`.
#[derive(Clone, Debug, Default)]
pub struct UserAgentMetadataOverride {
    pub brands: Vec<UserAgentBrand>,
    pub full_version_list: Vec<UserAgentBrand>,
    pub full_version: Option<String>,
    pub platform: Option<String>,
    pub platform_version: Option<String>,
    pub architecture: Option<String>,
    pub model: Option<String>,
    pub mobile: Option<bool>,
    pub bitness: Option<String>,
    pub wow64: Option<bool>,
    pub form_factors: Vec<String>,
}

impl UserAgentMetadataOverride {
    pub fn with_brand(mut self, brand: UserAgentBrand) -> Self {
        self.brands.push(brand);
        self
    }

    pub fn with_full_version_entry(mut self, brand: UserAgentBrand) -> Self {
        self.full_version_list.push(brand);
        self
    }

    pub fn with_full_version<T: Into<String>>(mut self, version: T) -> Self {
        self.full_version = Some(version.into());
        self
    }

    pub fn with_platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn with_platform_version<T: Into<String>>(mut self, version: T) -> Self {
        self.platform_version = Some(version.into());
        self
    }

    pub fn with_architecture<T: Into<String>>(mut self, arch: T) -> Self {
        self.architecture = Some(arch.into());
        self
    }

    pub fn with_model<T: Into<String>>(mut self, model: T) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_mobile(mut self, mobile: bool) -> Self {
        self.mobile = Some(mobile);
        self
    }

    pub fn with_bitness<T: Into<String>>(mut self, bitness: T) -> Self {
        self.bitness = Some(bitness.into());
        self
    }

    pub fn with_wow64(mut self, wow64: bool) -> Self {
        self.wow64 = Some(wow64);
        self
    }

    pub fn with_form_factor<T: Into<String>>(mut self, factor: T) -> Self {
        self.form_factors.push(factor.into());
        self
    }

    fn to_cdp(&self) -> UserAgentMetadata {
        UserAgentMetadata {
            brands: if self.brands.is_empty() {
                None
            } else {
                Some(self.brands.iter().map(|brand| brand.to_cdp()).collect())
            },
            full_version_list: if self.full_version_list.is_empty() {
                None
            } else {
                Some(
                    self.full_version_list
                        .iter()
                        .map(|brand| brand.to_cdp())
                        .collect(),
                )
            },
            full_version: self.full_version.clone(),
            platform: self.platform.clone().unwrap_or_default(),
            platform_version: self.platform_version.clone().unwrap_or_default(),
            architecture: self.architecture.clone().unwrap_or_default(),
            model: self.model.clone().unwrap_or_default(),
            mobile: self.mobile.unwrap_or(false),
            bitness: self.bitness.clone(),
            wow_64: self.wow64,
            form_factors: if self.form_factors.is_empty() {
                None
            } else {
                Some(self.form_factors.clone())
            },
        }
    }
}

/// Browser brand/version pair used in UA metadata.
#[derive(Clone, Debug)]
pub struct UserAgentBrand {
    pub brand: String,
    pub version: String,
}

impl UserAgentBrand {
    pub fn new<B: Into<String>, V: Into<String>>(brand: B, version: V) -> Self {
        Self {
            brand: brand.into(),
            version: version.into(),
        }
    }

    fn to_cdp(&self) -> UserAgentBrandVersion {
        UserAgentBrandVersion {
            brand: self.brand.clone(),
            version: self.version.clone(),
        }
    }
}

/// Controller that exposes Emulation domain commands for a page.
pub struct EmulationController {
    page: Arc<Page>,
}

impl EmulationController {
    pub(crate) fn new(page: Arc<Page>) -> Self {
        Self { page }
    }

    /// Applies a collection of emulation overrides to the target page.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{EmulationConfig, Geolocation, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let geolocation = Geolocation::new(37.7749, -122.4194).with_accuracy(1.0);
    /// let config = EmulationConfig::default().with_geolocation(geolocation);
    /// page.emulation().apply_config(&config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn apply_config(&self, config: &EmulationConfig) -> Result<()> {
        if let Some(geolocation) = &config.geolocation {
            self.set_geolocation(geolocation.clone()).await?;
        }
        if let Some(timezone) = &config.timezone_id {
            self.set_timezone(timezone).await?;
        }
        if let Some(locale) = &config.locale {
            self.set_locale(Some(locale.as_str())).await?;
        }
        if let Some(media) = &config.media {
            self.set_media(media.clone()).await?;
        }
        if let Some(user_agent) = &config.user_agent {
            self.set_user_agent(user_agent.clone()).await?;
        }
        Ok(())
    }

    /// Overrides the page's geolocation values until cleared.
    pub async fn set_geolocation(&self, geolocation: Geolocation) -> Result<()> {
        let method = SetGeolocationOverride {
            latitude: Some(geolocation.latitude),
            longitude: Some(geolocation.longitude),
            accuracy: geolocation.accuracy,
            altitude: geolocation.altitude,
            altitude_accuracy: geolocation.altitude_accuracy,
            heading: geolocation.heading,
            speed: geolocation.speed,
        };
        let _: SetGeolocationOverrideReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Removes any geolocation override previously applied.
    pub async fn clear_geolocation(&self) -> Result<()> {
        let method = ClearGeolocationOverride(None);
        let _: ClearGeolocationOverrideReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Sets the emulated timezone identifier (IANA format).
    pub async fn set_timezone<T: Into<String>>(&self, timezone_id: T) -> Result<()> {
        let method = SetTimezoneOverride {
            timezone_id: timezone_id.into(),
        };
        let _: SetTimezoneOverrideReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Resets the timezone override to the browser default.
    pub async fn reset_timezone(&self) -> Result<()> {
        self.set_timezone("").await
    }

    /// Overrides the page locale (for example "en-US").
    pub async fn set_locale(&self, locale: Option<&str>) -> Result<()> {
        let method = SetLocaleOverride {
            locale: locale.map(|value| value.to_string()),
        };
        let _: SetLocaleOverrideReturnObject = self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Applies media emulation settings, such as `prefers-color-scheme`.
    pub async fn set_media(&self, media: MediaEmulation) -> Result<()> {
        let method = SetEmulatedMedia {
            media: media.media_type.clone(),
            features: to_cdp_media_features(&media.features),
        };
        let _: SetEmulatedMediaReturnObject = self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Clears any previously applied media emulation overrides.
    pub async fn clear_media(&self) -> Result<()> {
        let method = SetEmulatedMedia {
            media: None,
            features: None,
        };
        let _: SetEmulatedMediaReturnObject = self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Overrides the user agent string and optional metadata.
    pub async fn set_user_agent(&self, override_data: UserAgentOverride) -> Result<()> {
        let method = SetUserAgentOverride {
            user_agent: override_data.user_agent,
            accept_language: override_data.accept_language,
            platform: override_data.platform,
            user_agent_metadata: override_data
                .metadata
                .as_ref()
                .map(UserAgentMetadataOverride::to_cdp),
        };
        let _: SetUserAgentOverrideReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }
}

fn to_cdp_media_features(features: &[MediaFeatureOverride]) -> Option<Vec<MediaFeature>> {
    if features.is_empty() {
        None
    } else {
        Some(
            features
                .iter()
                .map(|feature| MediaFeature {
                    name: feature.name.clone(),
                    value: feature.value.clone(),
                })
                .collect(),
        )
    }
}
