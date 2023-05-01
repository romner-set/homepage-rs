use crate::*;

#[derive(Properties, PartialEq)]
pub struct ServiceLinkProps {
    #[prop_or_default]
    name: &'static str,

    link_name: &'static str,

    #[prop_or_default]
    subdomain: &'static str,

    #[prop_or_default]
    cloudflare_access: bool,

    #[prop_or_default]
    children: Children,
}

#[function_component]
fn ServiceLink(props: &ServiceLinkProps) -> Html {
    let subdomain = if props.subdomain.is_empty() {props.link_name.to_lowercase()} else {props.subdomain.to_owned()};

    html! {
        <tr>
            <td>
                if props.name.is_empty() {
                    {"– "}{for props.children.iter()}{':'}
                } else {
                    {format!("– {}:", props.name)}
                }
            </td>
            <td>
                <a href={format!("https://{subdomain}.myalpine.live")}>
                    {props.link_name}
                </a>
            </td>
            if props.cloudflare_access {
                <td>{"(Cloudflare Access-secured)"}</td>
            }
        </tr>
    }
}

#[function_component()]
pub fn About() -> Html {
    html! {
        <>
            <title>{format!("::<> | About")}</title>
            <main>
                <h1 class="fish-head">{"about::"}<turbofish>{"myalpine.live"}</turbofish>{"()"}</h1>

                <div class="fish-body">
                    <p>
                        {"A personal website going through a "}
                        <a href="https://github.com/rapiz1/rathole">{"rathole"}</a>
                        {" reverse proxy to a self-hosted Alpine linux server running some stuff. Animation & styling were taken from "}
                        <a href="https://turbo.fish/">{"turbo.fish"}</a>
                        {", go check it out!"}
                    </p>

                    <strong>{"Services:"}</strong>
                    <table>
                        <tr/>
                        <ServiceLink name="Music server"  link_name="musikcube"/>
                        <ServiceLink name="Music sharing" link_name="slskd"/>
                        <ServiceLink name="Other sharing" link_name="qBittorrent"   subdomain="qb"/>
                        <ServiceLink name="File storage"  link_name="Nextcloud"/>
                        <ServiceLink name="Documents"     link_name="paperless-ngx" subdomain="paperless"/>
                        <ServiceLink name="Multimedia"    link_name="Emby"/>
                        <ServiceLink name="Factorio calc" link_name="fcalc"/>
                    </table>

                    <strong>{"Oracle CI VMs:"}</strong>
                    <table>
                        <tr/>
                        <ServiceLink name="Arch Linux x86 testing" link_name="vm1.myalpine.live" subdomain="vm1" cloudflare_access=true/>
                        <ServiceLink name="Alpine x86 file hosting & rathole" link_name="vm2.myalpine.live" subdomain="vm2"/>
                    </table>
                </div>
            </main>
            <Footer root_link=true/>
        </>
    }
}
