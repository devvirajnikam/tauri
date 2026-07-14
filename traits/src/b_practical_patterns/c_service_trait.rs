// Scenario:
// A higher-level service depends on lower-level behavior.
//
// Thinking:
// Passing traits into services makes the service testable and independent of the
// concrete implementation.

trait SmsGateway {
    fn send_sms(&self, phone: &str, message: &str);
}

struct ConsoleSmsGateway;

impl SmsGateway for ConsoleSmsGateway {
    fn send_sms(&self, phone: &str, message: &str) {
        println!("SMS to {}: {}", phone, message);
    }
}

struct OtpService<T: SmsGateway> {
    gateway: T,
}

impl<T: SmsGateway> OtpService<T> {
    fn send_otp(&self, phone: &str) {
        self.gateway.send_sms(phone, "Your OTP is 123456");
    }
}

pub fn run() {
    println!("\n7. Service dependency through trait");

    let service = OtpService {
        gateway: ConsoleSmsGateway,
    };

    service.send_otp("+91-9999999999");
}
