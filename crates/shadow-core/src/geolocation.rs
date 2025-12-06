//! IP Geolocation Service
//!
//! Provides geographic location data for IP addresses to enable:
//! - Optimal server routing
//! - Region-based matchmaking
//! - Fraud detection
//! - Analytics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// IP geolocation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    /// IP address
    pub ip: IpAddr,
    /// Country code (ISO 3166-1 alpha-2)
    pub country_code: String,
    /// Country name
    pub country_name: String,
    /// Region/State code
    pub region_code: Option<String>,
    /// Region/State name
    pub region_name: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Postal code
    pub postal_code: Option<String>,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Timezone
    pub timezone: Option<String>,
    /// ISP name
    pub isp: Option<String>,
    /// Organization
    pub organization: Option<String>,
    /// AS number
    pub asn: Option<u32>,
    /// Connection type
    pub connection_type: Option<ConnectionType>,
    /// Is VPN/Proxy
    pub is_proxy: bool,
    /// Is datacenter IP
    pub is_datacenter: bool,
    /// Threat score (0-100)
    pub threat_score: u8,
    /// Lookup timestamp
    pub timestamp: DateTime<Utc>,
}

impl GeoLocation {
    /// Create unknown location (for failed lookups)
    pub fn unknown(ip: IpAddr) -> Self {
        Self {
            ip,
            country_code: "XX".to_string(),
            country_name: "Unknown".to_string(),
            region_code: None,
            region_name: None,
            city: None,
            postal_code: None,
            latitude: 0.0,
            longitude: 0.0,
            timezone: None,
            isp: None,
            organization: None,
            asn: None,
            connection_type: None,
            is_proxy: false,
            is_datacenter: false,
            threat_score: 0,
            timestamp: Utc::now(),
        }
    }

    /// Check if this is a high-risk connection
    pub fn is_high_risk(&self) -> bool {
        self.is_proxy || self.is_datacenter || self.threat_score > 50
    }

    /// Get distance to another location in kilometers
    pub fn distance_to(&self, other: &GeoLocation) -> f64 {
        haversine_distance(self.latitude, self.longitude, other.latitude, other.longitude)
    }
}

/// Connection type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Residential connection
    Residential,
    /// Mobile/Cellular
    Mobile,
    /// Business/Corporate
    Business,
    /// Datacenter/Hosting
    Datacenter,
    /// Education/University
    Education,
    /// Government
    Government,
}

/// Server region for routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServerRegion {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Oceania,
    Africa,
    MiddleEast,
}

impl ServerRegion {
    /// Get region from country code
    pub fn from_country_code(code: &str) -> Self {
        match code.to_uppercase().as_str() {
            // North America
            "US" | "CA" | "MX" => ServerRegion::NorthAmerica,
            // South America
            "BR" | "AR" | "CL" | "CO" | "PE" | "VE" | "EC" | "UY" | "PY" | "BO" => {
                ServerRegion::SouthAmerica
            }
            // Europe
            "GB" | "DE" | "FR" | "IT" | "ES" | "PL" | "NL" | "BE" | "SE" | "NO" | "DK" | "FI"
            | "PT" | "AT" | "CH" | "CZ" | "RO" | "HU" | "IE" | "GR" | "UA" | "RU" | "BY" => {
                ServerRegion::Europe
            }
            // Asia
            "CN" | "JP" | "KR" | "IN" | "ID" | "TH" | "VN" | "PH" | "MY" | "SG" | "TW" | "HK" => {
                ServerRegion::Asia
            }
            // Oceania
            "AU" | "NZ" => ServerRegion::Oceania,
            // Middle East
            "AE" | "SA" | "IL" | "TR" | "EG" | "QA" | "KW" | "BH" | "OM" | "JO" | "LB" => {
                ServerRegion::MiddleEast
            }
            // Africa
            "ZA" | "NG" | "KE" | "GH" | "TZ" | "ET" | "UG" | "DZ" | "MA" | "TN" => {
                ServerRegion::Africa
            }
            _ => ServerRegion::Europe, // Default to Europe
        }
    }

    /// Get server endpoint for region
    pub fn server_endpoint(&self) -> &'static str {
        match self {
            ServerRegion::NorthAmerica => "na.shadow-ot.com",
            ServerRegion::SouthAmerica => "sa.shadow-ot.com",
            ServerRegion::Europe => "eu.shadow-ot.com",
            ServerRegion::Asia => "asia.shadow-ot.com",
            ServerRegion::Oceania => "oce.shadow-ot.com",
            ServerRegion::Africa => "af.shadow-ot.com",
            ServerRegion::MiddleEast => "me.shadow-ot.com",
        }
    }

    /// Get average latency estimation (ms) from source region
    pub fn estimated_latency_from(&self, source: ServerRegion) -> u32 {
        if *self == source {
            return 20; // Same region
        }
        
        match (source, self) {
            // Adjacent regions
            (ServerRegion::NorthAmerica, ServerRegion::SouthAmerica) => 80,
            (ServerRegion::NorthAmerica, ServerRegion::Europe) => 100,
            (ServerRegion::Europe, ServerRegion::MiddleEast) => 60,
            (ServerRegion::Europe, ServerRegion::Africa) => 80,
            (ServerRegion::Asia, ServerRegion::Oceania) => 80,
            (ServerRegion::Asia, ServerRegion::MiddleEast) => 70,
            // Cross-region
            (ServerRegion::NorthAmerica, ServerRegion::Asia) => 150,
            (ServerRegion::Europe, ServerRegion::Asia) => 130,
            (ServerRegion::SouthAmerica, ServerRegion::Europe) => 150,
            (ServerRegion::SouthAmerica, ServerRegion::Asia) => 250,
            (ServerRegion::Oceania, ServerRegion::Europe) => 250,
            (ServerRegion::Africa, ServerRegion::Asia) => 180,
            _ => 150, // Default
        }
    }
}

/// Geolocation service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoConfig {
    /// MaxMind GeoIP2 database path
    pub maxmind_db_path: Option<String>,
    /// IP-API endpoint (fallback)
    pub ipapi_endpoint: String,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Enable proxy detection
    pub enable_proxy_detection: bool,
    /// Block high-risk IPs
    pub block_high_risk: bool,
    /// High-risk threshold (0-100)
    pub high_risk_threshold: u8,
}

impl Default for GeoConfig {
    fn default() -> Self {
        Self {
            maxmind_db_path: None,
            ipapi_endpoint: "http://ip-api.com/json".to_string(),
            cache_ttl_seconds: 3600,
            enable_proxy_detection: true,
            block_high_risk: false,
            high_risk_threshold: 70,
        }
    }
}

/// Geolocation service
#[derive(Debug)]
pub struct GeoService {
    config: GeoConfig,
    cache: Arc<RwLock<GeoCache>>,
    server_locations: HashMap<ServerRegion, GeoLocation>,
}

impl GeoService {
    /// Create new geolocation service
    pub fn new(config: GeoConfig) -> Self {
        let mut server_locations = HashMap::new();
        
        // Define server locations for distance calculation
        server_locations.insert(ServerRegion::NorthAmerica, GeoLocation {
            ip: "0.0.0.0".parse().unwrap(),
            country_code: "US".to_string(),
            country_name: "United States".to_string(),
            region_code: Some("VA".to_string()),
            region_name: Some("Virginia".to_string()),
            city: Some("Ashburn".to_string()),
            postal_code: None,
            latitude: 39.0438,
            longitude: -77.4874,
            timezone: Some("America/New_York".to_string()),
            isp: None,
            organization: None,
            asn: None,
            connection_type: Some(ConnectionType::Datacenter),
            is_proxy: false,
            is_datacenter: true,
            threat_score: 0,
            timestamp: Utc::now(),
        });
        
        server_locations.insert(ServerRegion::Europe, GeoLocation {
            ip: "0.0.0.0".parse().unwrap(),
            country_code: "DE".to_string(),
            country_name: "Germany".to_string(),
            region_code: Some("HE".to_string()),
            region_name: Some("Hesse".to_string()),
            city: Some("Frankfurt".to_string()),
            postal_code: None,
            latitude: 50.1109,
            longitude: 8.6821,
            timezone: Some("Europe/Berlin".to_string()),
            isp: None,
            organization: None,
            asn: None,
            connection_type: Some(ConnectionType::Datacenter),
            is_proxy: false,
            is_datacenter: true,
            threat_score: 0,
            timestamp: Utc::now(),
        });
        
        server_locations.insert(ServerRegion::SouthAmerica, GeoLocation {
            ip: "0.0.0.0".parse().unwrap(),
            country_code: "BR".to_string(),
            country_name: "Brazil".to_string(),
            region_code: Some("SP".to_string()),
            region_name: Some("São Paulo".to_string()),
            city: Some("São Paulo".to_string()),
            postal_code: None,
            latitude: -23.5505,
            longitude: -46.6333,
            timezone: Some("America/Sao_Paulo".to_string()),
            isp: None,
            organization: None,
            asn: None,
            connection_type: Some(ConnectionType::Datacenter),
            is_proxy: false,
            is_datacenter: true,
            threat_score: 0,
            timestamp: Utc::now(),
        });
        
        server_locations.insert(ServerRegion::Asia, GeoLocation {
            ip: "0.0.0.0".parse().unwrap(),
            country_code: "SG".to_string(),
            country_name: "Singapore".to_string(),
            region_code: None,
            region_name: None,
            city: Some("Singapore".to_string()),
            postal_code: None,
            latitude: 1.3521,
            longitude: 103.8198,
            timezone: Some("Asia/Singapore".to_string()),
            isp: None,
            organization: None,
            asn: None,
            connection_type: Some(ConnectionType::Datacenter),
            is_proxy: false,
            is_datacenter: true,
            threat_score: 0,
            timestamp: Utc::now(),
        });
        
        Self {
            config,
            cache: Arc::new(RwLock::new(GeoCache::new())),
            server_locations,
        }
    }

    /// Lookup IP address
    pub async fn lookup(&self, ip: IpAddr) -> GeoLocation {
        // Check cache first
        if let Some(cached) = self.get_cached(&ip).await {
            return cached;
        }

        // Check if private IP
        if is_private_ip(&ip) {
            return GeoLocation::unknown(ip);
        }

        // Try MaxMind first, then fallback to IP-API
        let location = self.lookup_maxmind(&ip)
            .unwrap_or_else(|| GeoLocation::unknown(ip));

        // Cache the result
        self.cache_location(location.clone()).await;

        location
    }

    /// Get optimal server region for an IP
    pub async fn get_optimal_region(&self, ip: IpAddr) -> ServerRegion {
        let location = self.lookup(ip).await;
        
        // First, use country code for regional routing
        let region = ServerRegion::from_country_code(&location.country_code);
        
        // If we have coordinates, find closest server by distance
        if location.latitude != 0.0 || location.longitude != 0.0 {
            let mut best_region = region;
            let mut best_distance = f64::MAX;
            
            for (server_region, server_location) in &self.server_locations {
                let distance = location.distance_to(server_location);
                if distance < best_distance {
                    best_distance = distance;
                    best_region = *server_region;
                }
            }
            
            return best_region;
        }
        
        region
    }

    /// Check if IP should be blocked
    pub async fn should_block(&self, ip: IpAddr) -> bool {
        if !self.config.block_high_risk {
            return false;
        }

        let location = self.lookup(ip).await;
        location.threat_score > self.config.high_risk_threshold
    }

    /// Get all server regions with estimated latency
    pub async fn get_server_latencies(&self, ip: IpAddr) -> Vec<(ServerRegion, u32)> {
        let location = self.lookup(ip).await;
        let source_region = ServerRegion::from_country_code(&location.country_code);
        
        vec![
            (ServerRegion::NorthAmerica, source_region.estimated_latency_from(ServerRegion::NorthAmerica)),
            (ServerRegion::SouthAmerica, source_region.estimated_latency_from(ServerRegion::SouthAmerica)),
            (ServerRegion::Europe, source_region.estimated_latency_from(ServerRegion::Europe)),
            (ServerRegion::Asia, source_region.estimated_latency_from(ServerRegion::Asia)),
            (ServerRegion::Oceania, source_region.estimated_latency_from(ServerRegion::Oceania)),
            (ServerRegion::Africa, source_region.estimated_latency_from(ServerRegion::Africa)),
            (ServerRegion::MiddleEast, source_region.estimated_latency_from(ServerRegion::MiddleEast)),
        ]
    }

    /// Lookup using MaxMind database
    fn lookup_maxmind(&self, _ip: &IpAddr) -> Option<GeoLocation> {
        // In production, this would use maxminddb crate
        // For now, return None to use fallback
        None
    }

    /// Get cached location
    async fn get_cached(&self, ip: &IpAddr) -> Option<GeoLocation> {
        let cache = self.cache.read().await;
        cache.get(ip)
    }

    /// Cache a location
    async fn cache_location(&self, location: GeoLocation) {
        let mut cache = self.cache.write().await;
        cache.insert(location, self.config.cache_ttl_seconds);
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            entries: cache.entries.len(),
            hits: cache.hits,
            misses: cache.misses,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
}

/// IP geolocation cache
#[derive(Debug)]
struct GeoCache {
    entries: HashMap<IpAddr, CacheEntry>,
    hits: u64,
    misses: u64,
}

#[derive(Debug)]
struct CacheEntry {
    location: GeoLocation,
    expires_at: DateTime<Utc>,
}

impl GeoCache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn get(&self, ip: &IpAddr) -> Option<GeoLocation> {
        if let Some(entry) = self.entries.get(ip) {
            if entry.expires_at > Utc::now() {
                return Some(entry.location.clone());
            }
        }
        None
    }

    fn insert(&mut self, location: GeoLocation, ttl_seconds: u64) {
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl_seconds as i64);
        self.entries.insert(location.ip, CacheEntry {
            location,
            expires_at,
        });
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Calculate haversine distance between two points in kilometers
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    
    EARTH_RADIUS_KM * c
}

/// Check if IP is private (RFC 1918)
fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8
            octets[0] == 10
            // 172.16.0.0/12
            || (octets[0] == 172 && (16..=31).contains(&octets[1]))
            // 192.168.0.0/16
            || (octets[0] == 192 && octets[1] == 168)
            // 127.0.0.0/8 (loopback)
            || octets[0] == 127
        }
        IpAddr::V6(ipv6) => {
            ipv6.is_loopback() || ipv6.segments()[0] == 0xfe80 // Link-local
        }
    }
}

/// Country info for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryInfo {
    pub code: String,
    pub name: String,
    pub flag_emoji: String,
}

impl CountryInfo {
    /// Get country info from code
    pub fn from_code(code: &str) -> Self {
        let (name, flag) = match code.to_uppercase().as_str() {
            "US" => ("United States", "🇺🇸"),
            "BR" => ("Brazil", "🇧🇷"),
            "DE" => ("Germany", "🇩🇪"),
            "GB" => ("United Kingdom", "🇬🇧"),
            "PL" => ("Poland", "🇵🇱"),
            "SE" => ("Sweden", "🇸🇪"),
            "MX" => ("Mexico", "🇲🇽"),
            "AR" => ("Argentina", "🇦🇷"),
            "CL" => ("Chile", "🇨🇱"),
            "CO" => ("Colombia", "🇨🇴"),
            "FR" => ("France", "🇫🇷"),
            "IT" => ("Italy", "🇮🇹"),
            "ES" => ("Spain", "🇪🇸"),
            "PT" => ("Portugal", "🇵🇹"),
            "NL" => ("Netherlands", "🇳🇱"),
            "BE" => ("Belgium", "🇧🇪"),
            "AT" => ("Austria", "🇦🇹"),
            "CH" => ("Switzerland", "🇨🇭"),
            "RU" => ("Russia", "🇷🇺"),
            "UA" => ("Ukraine", "🇺🇦"),
            "CN" => ("China", "🇨🇳"),
            "JP" => ("Japan", "🇯🇵"),
            "KR" => ("South Korea", "🇰🇷"),
            "IN" => ("India", "🇮🇳"),
            "AU" => ("Australia", "🇦🇺"),
            "NZ" => ("New Zealand", "🇳🇿"),
            "CA" => ("Canada", "🇨🇦"),
            "SG" => ("Singapore", "🇸🇬"),
            "ZA" => ("South Africa", "🇿🇦"),
            "AE" => ("United Arab Emirates", "🇦🇪"),
            "SA" => ("Saudi Arabia", "🇸🇦"),
            "IL" => ("Israel", "🇮🇱"),
            "TR" => ("Turkey", "🇹🇷"),
            _ => ("Unknown", "🏳️"),
        };

        Self {
            code: code.to_uppercase(),
            name: name.to_string(),
            flag_emoji: flag.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_country() {
        assert_eq!(ServerRegion::from_country_code("US"), ServerRegion::NorthAmerica);
        assert_eq!(ServerRegion::from_country_code("BR"), ServerRegion::SouthAmerica);
        assert_eq!(ServerRegion::from_country_code("DE"), ServerRegion::Europe);
        assert_eq!(ServerRegion::from_country_code("JP"), ServerRegion::Asia);
        assert_eq!(ServerRegion::from_country_code("AU"), ServerRegion::Oceania);
    }

    #[test]
    fn test_haversine_distance() {
        // New York to London ~5570 km
        let distance = haversine_distance(40.7128, -74.0060, 51.5074, -0.1278);
        assert!((distance - 5570.0).abs() < 50.0);
    }

    #[test]
    fn test_private_ip() {
        assert!(is_private_ip(&"10.0.0.1".parse().unwrap()));
        assert!(is_private_ip(&"192.168.1.1".parse().unwrap()));
        assert!(is_private_ip(&"172.16.0.1".parse().unwrap()));
        assert!(is_private_ip(&"127.0.0.1".parse().unwrap()));
        assert!(!is_private_ip(&"8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn test_country_info() {
        let info = CountryInfo::from_code("BR");
        assert_eq!(info.name, "Brazil");
        assert_eq!(info.flag_emoji, "🇧🇷");
    }

    #[tokio::test]
    async fn test_geo_service() {
        let service = GeoService::new(GeoConfig::default());
        
        // Private IP should return unknown
        let location = service.lookup("192.168.1.1".parse().unwrap()).await;
        assert_eq!(location.country_code, "XX");
    }
}
