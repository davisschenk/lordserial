#![allow(dead_code)]
use derive_field::{DataPacket, FieldExtract};
use packet::Field;
use packet::RawField;
use serde::{self, Serialize};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Packet {
    BASE {},
    DM {},
    ESTIMATION {},
    SYSTEM {},
    IMU(imu_data::ImuPacket),
    GNSS(gnss_data::GnssPacket),
    FILTER(filter_data::FilterPacket),
}

impl Packet {
    pub fn new(packet: &packet::RawPacket) -> Self {
        match packet.header.descriptor {
            0x80 => Self::IMU(imu_data::ImuPacket::from_vec(&packet.payload.fields)),
            0x81 => Self::GNSS(gnss_data::GnssPacket::from_vec(&packet.payload.fields)),
            0x82 => Self::FILTER(filter_data::FilterPacket::from_vec(&packet.payload.fields)),
            0x01 => Self::BASE {},
            0x0C => Self::DM {},
            0x0D => Self::ESTIMATION {},
            0x7F => Self::SYSTEM {},
            _ => panic!("Not a data packet"),
        }
    }
}
mod imu_data {
    use super::*;

    #[derive(DataPacket, Debug, Serialize)]
    pub struct ImuPacket {
        #[serde(skip_serializing_if = "Option::is_none")]
        accelerometer: Option<ScaledAccelerometerVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gyro: Option<ScaledGyroVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        magnetometer: Option<ScaledMagnetometerVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pressure: Option<ScaledAmbientPressure>,

        #[serde(skip_serializing_if = "Option::is_none")]
        dtv: Option<DeltaThetaVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        dvv: Option<DeltaVelocityVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        orientation_matrix: Option<OrientationMatrix>,

        #[serde(skip_serializing_if = "Option::is_none")]
        quaternion: Option<Quaternion>,

        #[serde(skip_serializing_if = "Option::is_none")]
        euler_angles: Option<EulerAngles>,

        #[serde(skip_serializing_if = "Option::is_none")]
        north_vector: Option<StabilizedNorthVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        up_vector: Option<StabilizedUpVector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gps_correlation: Option<GpsCorrelationTimestamp>,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x04)]
    pub struct ScaledAccelerometerVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x05)]
    pub struct ScaledGyroVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x06)]
    pub struct ScaledMagnetometerVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x17)]
    pub struct ScaledAmbientPressure {
        ambient_pressure: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x07)]
    pub struct DeltaThetaVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x08)]
    pub struct DeltaVelocityVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x09)]
    pub struct OrientationMatrix {
        m11: f32,
        m12: f32,
        m13: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x0A)]
    pub struct Quaternion {
        q0: f32,
        q1: f32,
        q2: f32,
        q3: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x0C)]
    pub struct EulerAngles {
        roll: f32,
        pitch: f32,
        yaw: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x10)]
    pub struct StabilizedNorthVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x11)]
    pub struct StabilizedUpVector {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x12)]
    pub struct GpsCorrelationTimestamp {
        tow: f64,
        week: u16,
        flags: u16,
    }
}

mod gnss_data {
    use super::*;

    #[derive(DataPacket, Debug, Serialize)]
    pub struct GnssPacket {
        #[serde(skip_serializing_if = "Option::is_none")]
        llh: Option<LlhPosition>,

        #[serde(skip_serializing_if = "Option::is_none")]
        ecef_position: Option<EcefPosition>,

        #[serde(skip_serializing_if = "Option::is_none")]
        ned_velocity: Option<NedVelocity>,

        #[serde(skip_serializing_if = "Option::is_none")]
        ecef_velocity: Option<EcefVelocity>,

        #[serde(skip_serializing_if = "Option::is_none")]
        dop_data: Option<DopData>,

        #[serde(skip_serializing_if = "Option::is_none")]
        utc_time: Option<UtcTime>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gps_time: Option<GpsTime>,

        #[serde(skip_serializing_if = "Option::is_none")]
        clock_information: Option<ClockInformation>,

        #[serde(skip_serializing_if = "Option::is_none")]
        fix_information: Option<FixInformation>,

        #[serde(skip_serializing_if = "Option::is_none")]
        sv_information: Option<SpaceVehicleInformation>,

        #[serde(skip_serializing_if = "Option::is_none")]
        hardware_status: Option<HardwareStatus>,

        #[serde(skip_serializing_if = "Option::is_none")]
        dgnss_information: Option<DgnssInformation>,

        #[serde(skip_serializing_if = "Option::is_none")]
        dgnss_status: Option<DgnssStatus>,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x03)]
    pub struct LlhPosition {
        latitude: f64,
        longitude: f64,
        hae: f64,
        msl: f64,
        horizontal_accuracy: f32,
        vertical_accuracy: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x04)]
    pub struct EcefPosition {
        x: f64,
        y: f64,
        z: f64,
        position_accuracy: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x05)]
    pub struct NedVelocity {
        north: f32,
        east: f32,
        down: f32,
        speed: f32,
        ground_speed: f32,
        heading: f32,
        speed_accuracy: f32,
        heading_accuracy: f32,
        flags: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x06)]
    pub struct EcefVelocity {
        x: f32,
        y: f32,
        z: f32,
        velocity_accuracy: f32,
        flags: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x07)]
    pub struct DopData {
        gdop: f32,
        pdop: f32,
        hdop: f32,
        vdop: f32,
        tdop: f32,
        ndop: f32,
        edop: f32,
        flags: f32,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x08)]
    pub struct UtcTime {
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x09)]
    pub struct GpsTime {
        tow: f64,
        week: u16,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0A)]
    pub struct ClockInformation {
        bias: f64,
        drift: f64,
        accuracy_estimate: f64,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0B)]
    pub struct FixInformation {
        fix_type: u8,
        svs: u8,
        fix_flags: u16,
        valid_flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0C)]
    pub struct SpaceVehicleInformation {
        channel: u8,
        vehicle_id: u8,
        carrier_noise_ratio: u16,
        azimuth: i16,
        elevation: i16,
        vehicle_flags: u16,
        valid_flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0D)]
    pub struct HardwareStatus {
        sensor_state: u8,
        antenna_state: u8,
        antenna_power: u8,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0E)]
    pub struct DgnssInformation {
        newest_age: f32,
        base_station_id: i16,
        base_station_status: i16,
        dgnss_channels: u16,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x81, 0x0F)]
    pub struct DgnssStatus {
        vehicle_id: u8,
        age: f32,
        pseudorange_correction: f32,
        pseudorange_rate_correction: f32,
        flags: u16,
    }
}

mod filter_data {
    use super::*;

    #[derive(DataPacket, Debug, Serialize)]
    pub struct FilterPacket {
        #[serde(skip_serializing_if = "Option::is_none")]
        filter_status: Option<FilterStatus>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gps_time: Option<GpsTime>,

        #[serde(skip_serializing_if = "Option::is_none")]
        llh_position: Option<LlhPosition>,

        #[serde(skip_serializing_if = "Option::is_none")]
        llh_position_uncertainty: Option<LlhPositionUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        ned_velocity: Option<NedVelocity>,

        #[serde(skip_serializing_if = "Option::is_none")]
        ned_velocity_uncertainty: Option<NedVelocityUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        quanternion: Option<Quaternion>,

        #[serde(skip_serializing_if = "Option::is_none")]
        quanternion_uncertanity: Option<QuaternionUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        euler_angles: Option<EulerAngles>,

        #[serde(skip_serializing_if = "Option::is_none")]
        euler_angles_uncertainty: Option<EulerAnglesUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        orientation_matrix: Option<OrientationMatrix>,

        #[serde(skip_serializing_if = "Option::is_none")]
        angular_rate: Option<AngularRate>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gyro_bias: Option<GyroBias>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gyro_bias_uncertainty: Option<GyroBiasUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gyro_scale_factor: Option<GyroScaleFactor>,

        #[serde(skip_serializing_if = "Option::is_none")]
        gyro_scale_factor_uncertainty: Option<GyroScaleFactorUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        compensated_acceleration: Option<CompensatedAcceleration>,

        #[serde(skip_serializing_if = "Option::is_none")]
        linear_acceleration: Option<LinearAcceleration>,

        #[serde(skip_serializing_if = "Option::is_none")]
        accel_bias: Option<AccelBias>,

        #[serde(skip_serializing_if = "Option::is_none")]
        accel_bias_uncertainty: Option<AccelBiasUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        accel_scale_factor: Option<AccelScaleFactor>,

        #[serde(skip_serializing_if = "Option::is_none")]
        accel_scale_factor_uncertainty: Option<AccelScaleFactorUncertainty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pressure_altitude: Option<PressureAltitude>,

        #[serde(skip_serializing_if = "Option::is_none")]
        standard_atmosphere_model: Option<StandardAtmosphereModel>,
    }
    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x10)]
    pub struct FilterStatus {
        state: u16,
        dynamics: u16,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x11)]
    pub struct GpsTime {
        tow: f64,
        week: u16,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x01)]
    pub struct LlhPosition {
        latitude: f64,
        longitude: f64,
        hae: f64,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x08)]
    pub struct LlhPositionUncertainty {
        north: f32,
        east: f32,
        south: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x02)]
    pub struct NedVelocity {
        north: f32,
        east: f32,
        down: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x09)]
    pub struct NedVelocityUncertainty {
        north: f32,
        east: f32,
        down: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x03)]
    pub struct Quaternion {
        q0: f32,
        q1: f32,
        q2: f32,
        q3: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x12)]
    pub struct QuaternionUncertainty {
        q0: f32,
        q1: f32,
        q2: f32,
        q3: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x05)]
    pub struct EulerAngles {
        roll: f32,
        pitch: f32,
        yaw: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x0A)]
    pub struct EulerAnglesUncertainty {
        roll: f32,
        pitch: f32,
        yaw: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x04)]
    pub struct OrientationMatrix {
        m11: f32,
        m12: f32,
        m13: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m31: f32,
        m32: f32,
        m33: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x0E)]
    pub struct AngularRate {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x06)]
    pub struct GyroBias {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x0B)]
    pub struct GyroBiasUncertainty {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x16)]
    pub struct GyroScaleFactor {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x18)]
    pub struct GyroScaleFactorUncertainty {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x1C)]
    pub struct CompensatedAcceleration {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x0D)]
    pub struct LinearAcceleration {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x07)]
    pub struct AccelBias {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x0C)]
    pub struct AccelBiasUncertainty {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x17)]
    pub struct AccelScaleFactor {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x19)]
    pub struct AccelScaleFactorUncertainty {
        x: f32,
        y: f32,
        z: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x21)]
    pub struct PressureAltitude {
        altitude: f32,
        flags: u16,
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x82, 0x20)]
    pub struct StandardAtmosphereModel {
        geometric_altitude: f32,
        geopotential_altitude: f32,
        temperature: f32,
        pressure: f32,
        density: f32,
        flags: u16,
    }

    // TODO: Implement more types starting at gravity vector (0x82, 0x13)
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use packet::*;
    #[test]
    fn it_works() {
        let packet = Packet { header: Header { sync_one: 117, sync_two: 101, descriptor: 128 }, payload: Payload { length: 80, fields: vec![RawField { length: 6, descriptor: 23, data: vec![68, 81, 230, 22] }, RawField { length: 14, descriptor: 6, data: vec![61, 180, 25, 94, 62, 21, 152, 247, 62, 51, 29, 38] }, RawField { length: 14, descriptor: 4, data: vec![190, 22, 213, 242, 190, 232, 25, 3, 191, 96, 111, 155] }, RawField { length: 14, descriptor: 5, data: vec![187, 59, 37, 155, 186, 232, 40, 116, 187, 167, 160, 189] }, RawField { length: 18, descriptor: 10, data: vec![63, 105, 52, 83, 62, 68, 185, 120, 190, 29, 27, 178, 190, 169, 151, 49] }, RawField { length: 14, descriptor: 18, data: vec![64, 165, 30, 214, 4, 24, 147, 117, 0, 0, 0, 6] }] }, checksum: Checksum { msb: 192, lsb: 14 } };
        println!("{:?}", imu_data::ImuPacket::from_vec(&packet.payload.fields));
    }
}
*/
