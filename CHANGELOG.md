# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.1.0] - 2025-12-22

### 🚀 Features

- Add new components and update branding assets
- *(dev)* Update dependencies and migrate Wrangler config to TOML
- *(auth)* Add authentication API with Supabase integration
- Add error handling and routing improvements
- Add async rate limiting to API registration routes
- Add i18n, footer, and GitHub release download endpoint
- Add .env.local.example and update environment configs
- Add production env example and update config files
- Add local environment variables to wrangler.toml
- Add download button click handler in Hero.vue
- Add GitHub redirect to Hero component button
- Add Tailwind CSS and update UI components
- Update navigation to use RouterLink with active state
- Added login, logout, unregister routes and modified register
- /devpipeline route
- Add storage module and expand API endpoints
- Add resend confirmation email endpoint and cooldown
- Add CI and CD GitHub Actions workflows
- Add release automation and changelog config

### 🐛 Bug Fixes

- Add 200 status messages to English and Italian locales
- Increase canvas height by 10px in FlickeringBackground
- Adjust canvas sizing logic in FlickeringBackground
- Stream GitHub release asset downloads directly
- Fix spacing in template and add cursor style
- Trello iframe
- Add user existence check to registration endpoint
- Update manifest path in release workflow

### 🚜 Refactor

- *(test)* Migrate server to Rust with initial Cargo setup
- Refactor API routing and update build configs
- Move API modules to worker directory and update deps
- Update env example and wrangler.toml for local and prod
- Update env example and wrangler config for new vars
- Improve header background and add footer copyright
- Update environment variable names in example file
- Update building illustration and related components
- Added display_name query param
- Refactor auth verification to use confirm endpoint
