# There are no tests yet
%bcond_with check

Name:           i3rustus
Version:        0.3.0
Release:        2%{?dist}
Summary:        Lightweight implementation of i3status in rust
# Apache-2.0 OR BSL-1.0
# Apache-2.0 OR MIT
# MIT
# MIT OR Apache-2.0
License:        (Apache-2.0 OR BSL-1.0) AND (Apache-2.0 OR MIT) AND MIT AND (MIT OR Apache-2.0)
URL:            https://github.com/blinxen/%{name}
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24

%description
Lightweight implementation of i3status in rust

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/i3rustus

%prep
%autosetup -n %{name}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
* Tue Sep 10 2024 blinxen <h-k-81@hotmail.com> - 0.3.0-2
- Rebuild
* Sat Mar 02 2024 blinxen <h-k-81@hotmail.com> - 0.3.0-1
- Update to version 0.3.0
- Use netlink instead of NetworkManager dbus to get network information
* Mon Jun 26 2023 blinxen <h-k-81@hotmail.com> - 0.2.1-1
- Update to version 0.2.1
* Mon May 29 2023 blinxen <h-k-81@hotmail.com> - 0.2.0-1
- Update to version 0.2.0
- Add brightness indicator widget
- Small code cleanup
- First steps towards making i3rustus configurable
* Mon May 22 2023 blinxen <h-k-81@hotmail.com> - 0.1.0-1
- Initial package
