use hypertext::{prelude::*, Raw};

// Preheader padding: invisible Unicode characters (\u{00A0} = non-breaking space,
// \u{200C} = zero-width non-joiner) that fill the email preheader snippet in inbox
// previews, preventing body text from bleeding into the preview.
fn preheader_padding() -> String {
    "\u{00A0}\u{200C}".repeat(50)
}

/// Shared email shell — renders the outer HTML structure (DOCTYPE, head, body,
/// brand logo, container) so individual templates only provide inner content.
fn email_shell(title: &str, preheader: &str, inner_html: &str) -> String {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>(title)</title>
            </head>
            <body style="margin:0;padding:40px 16px;background:#f4f4f5;font-family:system-ui,-apple-system,sans-serif;">
                <span style="display:none;font-size:1px;color:#ffffff;max-height:0;max-width:0;opacity:0;overflow:hidden;mso-hide:all;">
                    (preheader)
                </span>
                <div style="max-width:480px;margin:0 auto;background:#ffffff;border-radius:8px;border:1px solid #e4e4e7;padding:40px;">
                    <img src="https://raw.githubusercontent.com/aginrocks/agin-auth/branding/branding/logo-text-black.png" alt="agin auth" width="148" height="33" style="display:block;margin:0 0 24px;">
                    (Raw::dangerously_create(inner_html))
                </div>
            </body>
        </html>
    }
    .render()
    .into_inner()
}

pub fn password_reset(reset_url: &str) -> String {
    let inner = rsx! {
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
    }
    .render()
    .into_inner();

    let preheader = format!(
        "Someone requested a password reset for your account. Click the link to set a new password. This link expires in 1 hour.{}",
        preheader_padding()
    );

    email_shell("Reset your password", &preheader, &inner)
}

pub fn email_confirmation(confirm_url: &str) -> String {
    let inner = rsx! {
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
    }
    .render()
    .into_inner();

    let preheader = format!(
        "Thanks for signing up! Click the button to verify your email address and activate your account.{}",
        preheader_padding()
    );

    email_shell("Confirm your email", &preheader, &inner)
}

pub fn login_notification(ip: &str) -> String {
    let inner = rsx! {
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
    }
    .render()
    .into_inner();

    let preheader = format!(
        "We detected a new login to your account. If this was you, no action is needed. If not, change your password immediately.{}",
        preheader_padding()
    );

    email_shell("New login detected", &preheader, &inner)
}

pub fn password_changed() -> String {
    let inner = rsx! {
        <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"Your password was changed"</h1>
        <p style="margin:0 0 28px;font-size:14px;color:#52525b;line-height:1.6;">
            "Your password was successfully changed. "
            "If you made this change, no further action is needed."
        </p>
        <p style="margin:0 0 0;font-size:14px;color:#52525b;line-height:1.6;">
            "If you didn't change your password, your account may be compromised. "
            "Please reset your password immediately and review your account activity."
        </p>
        <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
            "You are receiving this email because your account password was changed."
        </p>
    }
    .render()
    .into_inner();

    let preheader = format!(
        "Your password was successfully changed. If you didn't make this change, secure your account immediately.{}",
        preheader_padding()
    );

    email_shell("Password changed", &preheader, &inner)
}

pub fn factor_added(factor_name: &str) -> String {
    let inner = rsx! {
        <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"Security method added"</h1>
        <p style="margin:0 0 20px;font-size:14px;color:#52525b;line-height:1.6;">
            "The following authentication method was added to your account:"
        </p>
        <div style="padding:12px 16px;background:#f4f4f5;border-radius:6px;font-size:14px;font-weight:500;color:#09090b;margin-bottom:28px;">
            (factor_name)
        </div>
        <p style="margin:0 0 0;font-size:14px;color:#52525b;line-height:1.6;">
            "If you made this change, no further action is needed. "
            "If you didn't add this method, please review your account security immediately."
        </p>
        <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
            "You are receiving this email because a security method was added to your account."
        </p>
    }
    .render()
    .into_inner();

    let preheader = format!(
        "{factor_name} was added to your account. If you didn't make this change, secure your account immediately.{}",
        preheader_padding()
    );

    email_shell("Security method added", &preheader, &inner)
}

pub fn factor_removed(factor_name: &str) -> String {
    let inner = rsx! {
        <h1 style="margin:0 0 12px;font-size:20px;font-weight:600;color:#09090b;line-height:1.3;">"Security method removed"</h1>
        <p style="margin:0 0 20px;font-size:14px;color:#52525b;line-height:1.6;">
            "The following authentication method was removed from your account:"
        </p>
        <div style="padding:12px 16px;background:#f4f4f5;border-radius:6px;font-size:14px;font-weight:500;color:#09090b;margin-bottom:28px;">
            (factor_name)
        </div>
        <p style="margin:0 0 0;font-size:14px;color:#52525b;line-height:1.6;">
            "If you made this change, no further action is needed. "
            "If you didn't remove this method, please secure your account immediately."
        </p>
        <p style="margin:32px 0 0;font-size:12px;color:#a1a1aa;line-height:1.6;">
            "You are receiving this email because a security method was removed from your account."
        </p>
    }
    .render()
    .into_inner();

    let preheader = format!(
        "{factor_name} was removed from your account. If you didn't make this change, secure your account immediately.{}",
        preheader_padding()
    );

    email_shell("Security method removed", &preheader, &inner)
}
