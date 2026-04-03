#[cfg(test)]
mod tests {
    use mail::*;

    #[test]
    fn password_reset_contains_link() {
        let html = render_password_reset("https://example.com/reset?token=abc");
        assert!(html.contains("https://example.com/reset?token=abc"));
        assert!(html.contains("Reset your password"));
        assert!(html.contains("Reset password")); // button text
    }

    #[test]
    fn email_confirmation_contains_link() {
        let html = render_email_confirmation("https://example.com/confirm?token=xyz");
        assert!(html.contains("https://example.com/confirm?token=xyz"));
        assert!(html.contains("Confirm your email"));
        assert!(html.contains("Confirm email")); // button text
    }

    #[test]
    fn login_notification_contains_ip() {
        let html = render_login_notification("192.168.1.42");
        assert!(html.contains("192.168.1.42"));
        assert!(html.contains("New login to your account"));
    }

    #[test]
    fn password_changed_renders() {
        let html = render_password_changed();
        assert!(html.contains("Your password was changed"));
        assert!(html.contains("reset your password immediately"));
    }

    #[test]
    fn factor_added_contains_factor_name() {
        let html = render_factor_added("TOTP Authenticator");
        assert!(html.contains("TOTP Authenticator"));
        assert!(html.contains("Security method added"));
    }

    #[test]
    fn factor_added_security_key() {
        let html = render_factor_added("Security Key (WebAuthn)");
        assert!(html.contains("Security Key (WebAuthn)"));
    }

    #[test]
    fn factor_removed_contains_factor_name() {
        let html = render_factor_removed("TOTP Authenticator");
        assert!(html.contains("TOTP Authenticator"));
        assert!(html.contains("Security method removed"));
    }

    #[test]
    fn all_templates_have_logo() {
        let templates = [
            render_password_reset("https://example.com/reset"),
            render_email_confirmation("https://example.com/confirm"),
            render_login_notification("1.2.3.4"),
            render_password_changed(),
            render_factor_added("Test Factor"),
            render_factor_removed("Test Factor"),
        ];

        for html in &templates {
            assert!(
                html.contains("logo-text-black.png"),
                "Template missing logo"
            );
        }
    }

    #[test]
    fn all_templates_have_preheader() {
        let templates = [
            render_password_reset("https://example.com/reset"),
            render_email_confirmation("https://example.com/confirm"),
            render_login_notification("1.2.3.4"),
            render_password_changed(),
            render_factor_added("Test Factor"),
            render_factor_removed("Test Factor"),
        ];

        for html in &templates {
            // Preheader padding uses non-breaking spaces
            assert!(
                html.contains("\u{00A0}"),
                "Template missing preheader padding"
            );
        }
    }

    #[test]
    fn factor_name_is_html_escaped() {
        let html = render_factor_added("<script>alert('xss')</script>");
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }
}
