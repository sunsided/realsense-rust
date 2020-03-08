use crate::{
    device::Device,
    error::{ErrorChecker, Result as RsResult},
    kind::{CameraInfo, Extension, Rs2Option},
    processing_block_list::ProcessingBlockList,
    stream_profile_list::StreamProfileList,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

pub mod marker {
    use super::*;

    pub trait SensorKind {}
    pub trait NonAnySensorKind
    where
        Self: SensorKind,
    {
        const TYPE: Extension;
    }

    #[derive(Debug)]
    pub struct Any;
    impl SensorKind for Any {}

    #[derive(Debug)]
    pub struct Tm2;
    impl SensorKind for Tm2 {}
    impl NonAnySensorKind for Tm2 {
        const TYPE: Extension = Extension::Tm2Sensor;
    }

    #[derive(Debug)]
    pub struct Pose;
    impl SensorKind for Pose {}
    impl NonAnySensorKind for Pose {
        const TYPE: Extension = Extension::PoseSensor;
    }

    #[derive(Debug)]
    pub struct Color;
    impl SensorKind for Color {}
    impl NonAnySensorKind for Color {
        const TYPE: Extension = Extension::ColorSensor;
    }

    #[derive(Debug)]
    pub struct Depth;
    impl SensorKind for Depth {}
    impl NonAnySensorKind for Depth {
        const TYPE: Extension = Extension::DepthSensor;
    }

    #[derive(Debug)]
    pub struct Motion;
    impl SensorKind for Motion {}
    impl NonAnySensorKind for Motion {
        const TYPE: Extension = Extension::MotionSensor;
    }

    #[derive(Debug)]
    pub struct FishEye;
    impl SensorKind for FishEye {}
    impl NonAnySensorKind for FishEye {
        const TYPE: Extension = Extension::FishEyeSensor;
    }

    #[derive(Debug)]
    pub struct Software;
    impl SensorKind for Software {}
    impl NonAnySensorKind for Software {
        const TYPE: Extension = Extension::SoftwareSensor;
    }

    #[derive(Debug)]
    pub struct L500Depth;
    impl SensorKind for L500Depth {}
    impl NonAnySensorKind for L500Depth {
        const TYPE: Extension = Extension::L500DepthSensor;
    }

    #[derive(Debug)]
    pub struct DepthStereo;
    impl SensorKind for DepthStereo {}
    impl NonAnySensorKind for DepthStereo {
        const TYPE: Extension = Extension::DepthStereoSensor;
    }
}

#[derive(Debug)]
pub enum ExtendedSensor {
    Color(Sensor<marker::Color>),
    Depth(Sensor<marker::Depth>),
    DepthStereo(Sensor<marker::DepthStereo>),
    L500Depth(Sensor<marker::L500Depth>),
    Motion(Sensor<marker::Motion>),
    FishEye(Sensor<marker::FishEye>),
    Software(Sensor<marker::Software>),
    Pose(Sensor<marker::Pose>),
    Tm2(Sensor<marker::Tm2>),
    Other(Sensor<marker::Any>),
}

/// Represents a sensor on device.
#[derive(Debug)]
pub struct Sensor<Kind>
where
    Kind: marker::SensorKind,
{
    ptr: NonNull<realsense_sys::rs2_sensor>,
    _phantom: PhantomData<Kind>,
}

impl<Kind> Sensor<Kind>
where
    Kind: marker::SensorKind,
{
    /// Gets the corresponding device for sensor.
    pub fn device(&self) -> RsResult<Device> {
        let device = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_create_device_from_sensor(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Device::from_ptr(NonNull::new(ptr).unwrap())
        };
        Ok(device)
    }

    /// Gets an attribute on sensor.
    ///
    /// It will return error if the attribute is not available on sensor.
    pub fn get_option(&self, option: Rs2Option) -> RsResult<f32> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_option(
                self.ptr.as_ptr().cast::<realsense_sys::rs2_options>(),
                option as realsense_sys::rs2_option,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val)
        }
    }

    // pub fn set_option(&mut self, option: Rs2Option, value: f32) -> RsResult<()> {
    //     unsafe {
    //         let mut checker = ErrorChecker::new();
    //         let val = realsense_sys::rs2_set_option(
    //             self.ptr.as_ptr().cast::<realsense_sys::rs2_options>(),
    //             option as realsense_sys::rs2_option,
    //             value,
    //             checker.inner_mut_ptr(),
    //         );
    //         checker.check()?;
    //     }
    //     Ok(())
    // }

    /// List stream profiles on sensor.
    pub fn stream_profiles(&self) -> RsResult<StreamProfileList> {
        let list = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr =
                realsense_sys::rs2_get_stream_profiles(self.ptr.as_ptr(), checker.inner_mut_ptr());
            checker.check()?;
            StreamProfileList::from_ptr(NonNull::new(ptr).unwrap())
        };
        Ok(list)
    }

    pub fn recommended_processing_blocks(&self) -> RsResult<ProcessingBlockList> {
        let list = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_recommended_processing_blocks(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            ProcessingBlockList::from_ptr(NonNull::new(ptr).unwrap())
        };
        Ok(list)
    }

    pub fn name(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::Name)
    }

    pub fn serial_number(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::SerialNumber)
    }

    pub fn recommended_firmware_version(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::RecommendedFirmwareVersion)
    }

    pub fn physical_port(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::PhysicalPort)
    }

    pub fn debug_op_code(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::DebugOpCode)
    }

    pub fn advanced_mode(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::AdvancedMode)
    }

    pub fn product_id(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::ProductId)
    }

    pub fn camera_locked(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::CameraLocked)
    }

    pub fn usb_type_descriptor(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::UsbTypeDescriptor)
    }

    pub fn product_line(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::ProductLine)
    }

    pub fn asic_serial_number(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::AsicSerialNumber)
    }

    pub fn firmware_update_id(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::FirmwareUpdateId)
    }

    pub fn count(&self) -> RsResult<&CStr> {
        self.info(CameraInfo::Count)
    }

    pub fn info(&self, kind: CameraInfo) -> RsResult<&CStr> {
        let ptr = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_sensor_info(
                self.ptr.as_ptr(),
                kind as realsense_sys::rs2_camera_info,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            ptr
        };
        let string = unsafe { CStr::from_ptr(ptr) };
        Ok(string)
    }

    pub fn is_info_supported(&self, kind: CameraInfo) -> RsResult<bool> {
        let val = unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_supports_sensor_info(
                self.ptr.as_ptr(),
                kind as realsense_sys::rs2_camera_info,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            val
        };
        Ok(val != 0)
    }

    pub(crate) unsafe fn take(mut self) -> NonNull<realsense_sys::rs2_sensor> {
        let ptr = std::mem::replace(&mut self.ptr, MaybeUninit::uninit().assume_init());
        std::mem::forget(self);
        ptr
    }

    pub(crate) unsafe fn from_ptr(ptr: NonNull<realsense_sys::rs2_sensor>) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl Sensor<marker::Any> {
    pub fn is_extendable_to(&self, extension: Extension) -> RsResult<bool> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_is_sensor_extendable_to(
                self.ptr.as_ptr(),
                extension as realsense_sys::rs2_extension,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val != 0)
        }
    }

    /// Try to change the type of sensor.
    pub fn try_extend_to<NewKind>(self) -> RsResult<Result<Sensor<NewKind>, Self>>
    where
        NewKind: marker::NonAnySensorKind,
    {
        if self.is_extendable_to(NewKind::TYPE)? {
            let ptr = unsafe { self.take() };
            let sensor = Sensor {
                ptr,
                _phantom: PhantomData,
            };
            Ok(Ok(sensor))
        } else {
            Ok(Err(self))
        }
    }

    pub fn try_extend(self) -> RsResult<ExtendedSensor> {
        let sensor_any = self;

        let sensor_any = match sensor_any.try_extend_to::<marker::DepthStereo>()? {
            Ok(sensor) => return Ok(ExtendedSensor::DepthStereo(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Depth>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Depth(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::L500Depth>()? {
            Ok(sensor) => return Ok(ExtendedSensor::L500Depth(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Color>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Color(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Motion>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Motion(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::FishEye>()? {
            Ok(sensor) => return Ok(ExtendedSensor::FishEye(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Software>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Software(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Pose>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Pose(sensor)),
            Err(sensor) => sensor,
        };

        let sensor_any = match sensor_any.try_extend_to::<marker::Tm2>()? {
            Ok(sensor) => return Ok(ExtendedSensor::Tm2(sensor)),
            Err(sensor) => sensor,
        };

        Ok(ExtendedSensor::Other(sensor_any))
    }
}

impl Sensor<marker::Depth> {
    /// Gets the depth units of depth sensor.
    pub fn depth_units(&self) -> RsResult<f32> {
        self.get_option(Rs2Option::DepthUnits)
    }
}

unsafe impl<Kind> Send for Sensor<Kind> where Kind: marker::SensorKind {}

impl<Kind> Drop for Sensor<Kind>
where
    Kind: marker::SensorKind,
{
    fn drop(&mut self) {
        unsafe {
            realsense_sys::rs2_delete_sensor(self.ptr.as_ptr());
        }
    }
}
