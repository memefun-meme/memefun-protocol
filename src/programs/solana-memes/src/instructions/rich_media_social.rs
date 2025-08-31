use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

// Initialize rich media system
#[derive(Accounts)]
pub struct InitializeRichMediaSystem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<RichMediaSystem>(),
        seeds = [b"rich_media_system"],
        bump
    )]
    pub rich_media: Account<'info, RichMediaSystem>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_rich_media_system(ctx: Context<InitializeRichMediaSystem>) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    let clock = Clock::get()?;
    
    rich_media.authority = ctx.accounts.authority.key();
    rich_media.storage_usage = 0;
    rich_media.storage_limit = 1_000_000_000; // 1GB limit
    rich_media.created_at = clock.unix_timestamp;
    rich_media.updated_at = clock.unix_timestamp;
    
    // Initialize customization options
    rich_media.customization_options = CustomizationOptions {
        allow_custom_css: true,
        allow_custom_js: false, // Security: disable JS by default
        allow_external_embeds: true,
        allow_analytics: true,
        allow_social_integration: true,
    };
    
    // Initialize auto-posting config
    rich_media.auto_posting = AutoPostingConfig {
        enabled: false,
        platforms: Vec::new(),
        posting_schedule: PostingSchedule {
            frequency: PostingFrequency::Daily,
            preferred_times: vec![9, 12, 18], // 9 AM, 12 PM, 6 PM
            timezone: "UTC".to_string(),
            auto_schedule: true,
        },
        content_templates: Vec::new(),
        auto_hashtags: true,
        mention_creator: true,
    };
    
    // Initialize social metrics
    rich_media.social_metrics = SocialMetrics {
        total_followers: 0,
        total_engagement: 0,
        average_engagement_rate: 0.0,
        total_posts: 0,
        reach_impressions: 0,
        platform_breakdown: Vec::new(),
    };
    
    Ok(())
}

// Upload media file
#[derive(Accounts)]
pub struct UploadMediaFile<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn upload_media_file(
    ctx: Context<UploadMediaFile>,
    file_type: MediaType,
    file_uri: String,
    file_size: u64,
    title: String,
    description: String,
    tags: Vec<String>,
    license: String,
    is_public: bool,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    let clock = Clock::get()?;
    
    // Check storage limit
    require!(
        rich_media.storage_usage + file_size <= rich_media.storage_limit,
        CustomError::StorageLimitExceeded
    );
    
    // Validate file URI
    require!(
        file_uri.starts_with("https://") || file_uri.starts_with("ipfs://"),
        CustomError::InvalidFileUri
    );
    
    // Validate file size (max 10MB)
    require!(file_size <= 10_000_000, CustomError::FileTooLarge);
    
    // Create media file
    let file_id = rich_media.media_files.len() as u64 + 1;
    let media_file = MediaFile {
        file_id,
        token_mint: ctx.accounts.token_metadata.mint,
        file_type,
        file_uri,
        file_size,
        upload_time: clock.unix_timestamp,
        is_public,
        metadata: MediaMetadata {
            title,
            description,
            tags,
            creator: ctx.accounts.authority.key(),
            license,
            dimensions: None, // Would be extracted from file in real implementation
            duration: None,   // Would be extracted from file in real implementation
        },
    };
    
    rich_media.media_files.push(media_file);
    rich_media.storage_usage += file_size;
    rich_media.updated_at = clock.unix_timestamp;
    
    msg!("Media file uploaded: ID {}, Size: {} bytes", file_id, file_size);
    
    Ok(())
}

// Create or update token page
#[derive(Accounts)]
pub struct CreateTokenPage<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn create_token_page(
    ctx: Context<CreateTokenPage>,
    theme: PageTheme,
    layout: PageLayout,
    primary_color: String,
    secondary_color: String,
    background_color: String,
    text_color: String,
    accent_color: String,
    seo_title: String,
    seo_description: String,
    seo_keywords: Vec<String>,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    let clock = Clock::get()?;
    
    // Create page configuration
    let page_config = PageConfig {
        theme,
        layout,
        color_scheme: ColorScheme {
            primary_color,
            secondary_color,
            background_color,
            text_color,
            accent_color,
        },
        custom_css: None,
        seo_settings: SEOSettings {
            title: seo_title,
            description: seo_description,
            keywords: seo_keywords,
            og_image: None,
            twitter_card: None,
        },
    };
    
    // Find existing page or create new one
    let page_index = rich_media.token_pages
        .iter()
        .position(|p| p.token_mint == ctx.accounts.token_metadata.mint);
    
    if let Some(index) = page_index {
        // Update existing page
        let page = &mut rich_media.token_pages[index];
        page.page_config = page_config;
        page.last_updated = clock.unix_timestamp;
    } else {
        // Create new page
        let token_page = TokenPage {
            token_mint: ctx.accounts.token_metadata.mint,
            creator: ctx.accounts.token_metadata.creator,
            page_config,
            media_gallery: Vec::new(),
            social_links: Vec::new(),
            custom_sections: Vec::new(),
            analytics: PageAnalytics {
                total_views: 0,
                unique_visitors: 0,
                average_time_on_page: 0.0,
                bounce_rate: 0.0,
                social_shares: 0,
                media_downloads: 0,
            },
            last_updated: clock.unix_timestamp,
        };
        
        rich_media.token_pages.push(token_page);
    }
    
    rich_media.updated_at = clock.unix_timestamp;
    
    msg!("Token page created/updated for token: {}", ctx.accounts.token_metadata.mint);
    
    Ok(())
}

// Add social link to token page
#[derive(Accounts)]
pub struct AddSocialLink<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn add_social_link(
    ctx: Context<AddSocialLink>,
    platform: SocialPlatform,
    url: String,
    follower_count: u32,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Validate URL
    require!(
        url.starts_with("https://") || url.starts_with("http://"),
        CustomError::InvalidSocialUrl
    );
    
    // Find token page
    let page_index = rich_media.token_pages
        .iter()
        .position(|p| p.token_mint == ctx.accounts.token_metadata.mint)
        .ok_or(CustomError::TokenPageNotFound)?;
    
    let page = &mut rich_media.token_pages[page_index];
    
    // Check if social link already exists for this platform
    let existing_index = page.social_links
        .iter()
        .position(|link| link.platform == platform);
    
    let social_link = SocialLink {
        platform,
        url,
        is_verified: false, // Would be verified through external API
        follower_count,
    };
    
    if let Some(index) = existing_index {
        // Update existing link
        page.social_links[index] = social_link;
    } else {
        // Add new link
        page.social_links.push(social_link);
    }
    
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Social link added/updated for token: {}", ctx.accounts.token_metadata.mint);
    
    Ok(())
}

// Add custom section to token page
#[derive(Accounts)]
pub struct AddCustomSection<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn add_custom_section(
    ctx: Context<AddCustomSection>,
    title: String,
    content: String,
    section_type: SectionType,
    order: u32,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Validate content length
    require!(content.len() <= 10000, CustomError::ContentTooLong);
    require!(title.len() <= 200, CustomError::TitleTooLong);
    
    // Find token page
    let page_index = rich_media.token_pages
        .iter()
        .position(|p| p.token_mint == ctx.accounts.token_metadata.mint)
        .ok_or(CustomError::TokenPageNotFound)?;
    
    let page = &mut rich_media.token_pages[page_index];
    
    let section_id = page.custom_sections.len() as u64 + 1;
    let custom_section = CustomSection {
        section_id,
        title,
        content,
        section_type,
        is_visible: true,
        order,
    };
    
    page.custom_sections.push(custom_section);
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Custom section added to token page: {}", ctx.accounts.token_metadata.mint);
    
    Ok(())
}

// Connect social media account
#[derive(Accounts)]
pub struct ConnectSocialAccount<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
}

pub fn connect_social_account(
    ctx: Context<ConnectSocialAccount>,
    platform: SocialPlatform,
    api_key: String,
    api_secret: String,
    access_token: String,
    sync_frequency: i64,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    let clock = Clock::get()?;
    
    // Validate API credentials (simplified - in real implementation, would verify with platform)
    require!(!api_key.is_empty(), CustomError::InvalidApiCredentials);
    require!(!api_secret.is_empty(), CustomError::InvalidApiCredentials);
    require!(!access_token.is_empty(), CustomError::InvalidApiCredentials);
    
    // Check if platform already connected
    let existing_index = rich_media.social_integrations
        .iter()
        .position(|integration| integration.platform == platform);
    
    let social_integration = SocialIntegration {
        platform,
        api_key,
        api_secret,
        access_token,
        is_connected: true,
        last_sync: clock.unix_timestamp,
        sync_frequency,
    };
    
    if let Some(index) = existing_index {
        // Update existing integration
        rich_media.social_integrations[index] = social_integration;
    } else {
        // Add new integration
        rich_media.social_integrations.push(social_integration);
    }
    
    rich_media.updated_at = clock.unix_timestamp;
    
    msg!("Social account connected for platform");
    
    Ok(())
}

// Enable auto-posting
#[derive(Accounts)]
pub struct EnableAutoPosting<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
}

pub fn enable_auto_posting(
    ctx: Context<EnableAutoPosting>,
    platforms: Vec<SocialPlatform>,
    frequency: PostingFrequency,
    preferred_times: Vec<u8>,
    timezone: String,
    auto_hashtags: bool,
    mention_creator: bool,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Validate preferred times (0-23 hours)
    for time in &preferred_times {
        require!(*time <= 23, CustomError::InvalidTimeFormat);
    }
    
    // Validate timezone (simplified)
    require!(!timezone.is_empty(), CustomError::InvalidTimezone);
    
    rich_media.auto_posting.enabled = true;
    rich_media.auto_posting.platforms = platforms;
    rich_media.auto_posting.posting_schedule = PostingSchedule {
        frequency,
        preferred_times,
        timezone,
        auto_schedule: true,
    };
    rich_media.auto_posting.auto_hashtags = auto_hashtags;
    rich_media.auto_posting.mention_creator = mention_creator;
    
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Auto-posting enabled for {} platforms", rich_media.auto_posting.platforms.len());
    
    Ok(())
}

// Create content template for auto-posting
#[derive(Accounts)]
pub struct CreateContentTemplate<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
}

pub fn create_content_template(
    ctx: Context<CreateContentTemplate>,
    name: String,
    content: String,
    variables: Vec<String>,
    platform: SocialPlatform,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Validate template
    require!(content.len() <= 1000, CustomError::ContentTooLong);
    require!(name.len() <= 100, CustomError::TitleTooLong);
    
    // Validate variables (should be in format {variable_name})
    for variable in &variables {
        require!(
            variable.starts_with('{') && variable.ends_with('}'),
            CustomError::InvalidVariableFormat
        );
    }
    
    let template_id = rich_media.auto_posting.content_templates.len() as u64 + 1;
    let content_template = ContentTemplate {
        template_id,
        name,
        content,
        variables,
        platform,
    };
    
    rich_media.auto_posting.content_templates.push(content_template);
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Content template created: {}", template_id);
    
    Ok(())
}

// Update page analytics
#[derive(Accounts)]
pub struct UpdatePageAnalytics<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn update_page_analytics(
    ctx: Context<UpdatePageAnalytics>,
    total_views: u64,
    unique_visitors: u64,
    average_time_on_page: f64,
    bounce_rate: f64,
    social_shares: u64,
    media_downloads: u64,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Find token page
    let page_index = rich_media.token_pages
        .iter()
        .position(|p| p.token_mint == ctx.accounts.token_metadata.mint)
        .ok_or(CustomError::TokenPageNotFound)?;
    
    let page = &mut rich_media.token_pages[page_index];
    
    // Update analytics
    page.analytics = PageAnalytics {
        total_views,
        unique_visitors,
        average_time_on_page,
        bounce_rate,
        social_shares,
        media_downloads,
    };
    
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Page analytics updated for token: {}", ctx.accounts.token_metadata.mint);
    
    Ok(())
}

// Update social metrics
#[derive(Accounts)]
pub struct UpdateSocialMetrics<'info> {
    #[account(mut, has_one = authority)]
    pub rich_media: Account<'info, RichMediaSystem>,
    pub authority: Signer<'info>,
}

pub fn update_social_metrics(
    ctx: Context<UpdateSocialMetrics>,
    total_followers: u64,
    total_engagement: u64,
    total_posts: u64,
    reach_impressions: u64,
) -> Result<()> {
    let rich_media = &mut ctx.accounts.rich_media;
    
    // Calculate engagement rate
    let average_engagement_rate = if total_followers > 0 {
        total_engagement as f64 / total_followers as f64
    } else {
        0.0
    };
    
    rich_media.social_metrics = SocialMetrics {
        total_followers,
        total_engagement,
        average_engagement_rate,
        total_posts,
        reach_impressions,
        platform_breakdown: Vec::new(), // Would be populated from individual platform data
    };
    
    rich_media.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Social metrics updated: {} followers, {} engagement", total_followers, total_engagement);
    
    Ok(())
}
