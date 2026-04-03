use hypertext::prelude::*;

pub fn password_reset(app_name: &str, reset_url: &str) -> String {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>"Reset your password"</title>
            </head>
            <body style="margin:0;padding:40px 16px;background:#f4f4f5;font-family:system-ui,-apple-system,sans-serif;">
                <span style="display:none;font-size:1px;color:#ffffff;max-height:0;max-width:0;opacity:0;overflow:hidden;mso-hide:all;">
                    "Someone requested a password reset for your account. Click the link to set a new password. This link expires in 1 hour.\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}"
                </span>
                <div style="max-width:480px;margin:0 auto;background:#ffffff;border-radius:8px;border:1px solid #e4e4e7;padding:40px;">
                    <img src="https://raw.githubusercontent.com/aginrocks/agin-auth/branding/branding/logo-text-black.png" alt="agin auth" width="148" height="33" style="display:block;margin:0 0 24px;">
                    <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"Reset your password"</h1>
                    <p style="margin:0 0 28px;font-size:14px;color:#52525b;line-height:1.6;">
                        "Someone requested a password reset for your account. "
                        "Click the button below to set a new password. "
                        "This link expires in 1 hour."
                    </p>
                    <a href={reset_url} style="display:inline-block;padding:10px 20px;background:#09090b;color:#ffffff;text-decoration:none;border-radius:6px;font-size:14px;font-weight:500;">
                        "Reset password"
                    </a>
                    <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
                        "If you didn't request a password reset, you can safely ignore this email. "
                        "Your password will not change."
                    </p>
                </div>
            </body>
        </html>
    }
    .render()
    .into_inner()
}

pub fn email_confirmation(app_name: &str, confirm_url: &str) -> String {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>"Confirm your email"</title>
            </head>
            <body style="margin:0;padding:40px 16px;background:#f4f4f5;font-family:system-ui,-apple-system,sans-serif;">
                <span style="display:none;font-size:1px;color:#ffffff;max-height:0;max-width:0;opacity:0;overflow:hidden;mso-hide:all;">
                    "Thanks for signing up! Click the button to verify your email address and activate your account.\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}"
                </span>
                <div style="max-width:480px;margin:0 auto;background:#ffffff;border-radius:8px;border:1px solid #e4e4e7;padding:40px;">
                    <img src="https://raw.githubusercontent.com/aginrocks/agin-auth/branding/branding/logo-text-black.png" alt="agin auth" width="148" height="33" style="display:block;margin:0 0 24px;">
                    <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"Confirm your email address"</h1>
                    <p style="margin:0 0 28px;font-size:14px;color:#52525b;line-height:1.6;">
                        "Thanks for signing up. Click the button below to verify your email address."
                    </p>
                    <a href={confirm_url} style="display:inline-block;padding:10px 20px;background:#09090b;color:#ffffff;text-decoration:none;border-radius:6px;font-size:14px;font-weight:500;">
                        "Confirm email"
                    </a>
                    <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
                        "If you didn't create an account, you can safely ignore this email."
                    </p>
                </div>
            </body>
        </html>
    }
    .render()
    .into_inner()
}

pub fn login_notification(app_name: &str, ip: &str) -> String {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>"New login detected"</title>
            </head>
            <body style="margin:0;padding:40px 16px;background:#f4f4f5;font-family:system-ui,-apple-system,sans-serif;">
                <span style="display:none;font-size:1px;color:#ffffff;max-height:0;max-width:0;opacity:0;overflow:hidden;mso-hide:all;">
                    "We detected a new login to your account. If this was you, no action is needed. If not, change your password immediately.\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}\u{00A0}\u{200C}"
                </span>
                <div style="max-width:480px;margin:0 auto;background:#ffffff;border-radius:8px;border:1px solid #e4e4e7;padding:40px;">
                    <img src="https://raw.githubusercontent.com/aginrocks/agin-auth/branding/branding/logo-text-black.png" alt="agin auth" width="148" height="33" style="display:block;margin:0 0 24px;">
                    <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"New login to your account"</h1>
                    <p style="margin:0 0 20px;font-size:14px;color:#52525b;line-height:1.6;">
                        "We detected a new login to your account from the following IP address:"
                    </p>
                    <div style="padding:12px 16px;background:#f4f4f5;border-radius:6px;font-family:monospace;font-size:14px;color:#09090b;margin-bottom:28px;">
                        (ip)
                    </div>
                    <p style="margin:0 0 0;font-size:14px;color:#52525b;line-height:1.6;">
                        "If this was you, no action is needed. "
                        "If you don't recognize this login, please change your password immediately."
                    </p>
                    <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
                        "You are receiving this email because login notifications are enabled for your account."
                    </p>
                </div>
            </body>
        </html>
    }
    .render()
    .into_inner()
}
