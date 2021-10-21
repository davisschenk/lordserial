#![allow(dead_code)]
use derive_field::FieldExtract;
use serde::Serialize;


mod imu_data {
    use super::*;

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x04)]
    struct ScaledAccelerometerVector {
        x: f32,
        y: f32,
        z: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x05)]
    struct ScaledGyroVector {
        x: f32,
        y: f32,
        z: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x06)]
    struct ScaledMagnetometerVector {
        x: f32,
        y: f32,
        z: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x17)]
    struct ScaledAmbientPressure {
        ambient_pressure: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x07)]
    struct DeltaThetaVector {
        x: f32,
        y: f32,
        z: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x08)]
    struct DeltaVelocityVector {
        x: f32,
        y: f32,
        z: f32
    }

    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x09)]
    struct OrientationMatrix {
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
    struct Quaternion {
        q0: f32,
        q1: f32,
        q2: f32,
        q3: f32
    }


    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x0C)]
    struct EulerAngles {
        roll: f32,
        pitch: f32,
        yaw: f32
    }


    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x10)]
    struct StabilizedNorthVector {
        x: f32,
        y: f32,
        z: f32
    }


    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x11)]
    struct StabilizedUpVector {
        x: f32,
        y: f32,
        z: f32
    }

    
    #[derive(FieldExtract, Debug, Serialize)]
    #[descriptor(0x80, 0x12)]
    struct GpsCorrelationTimestamp {
        tow: f64,
        week: u16,
        flags: u16
    }
}

